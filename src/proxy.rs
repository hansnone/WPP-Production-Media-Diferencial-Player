//! Generación de vídeo proxy a partir de secuencias EXR (FFmpeg externo, FFV1 sin pérdidas).
//!
//! Escribe una lista concat (`exr_list.txt`), ordena los EXR por nombre de fichero y deja
//! el resultado en `proxy.mkv` dentro del directorio temporal indicado.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use parking_lot::Mutex;

/// Concat list filename and output video filename used in the temp dir.
pub const PROXY_VIDEO_FILENAME: &str = "proxy.mkv";
const EXR_LIST_FILENAME: &str = "exr_list.txt";

/// Ordena rutas EXR por nombre de fichero (mismo criterio que `ls` lexicográfico en el nombre).
fn sort_exr_paths_by_file_name(paths: &mut [PathBuf]) {
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
}

/// Collect and sort EXR paths: from a directory (list .exr inside) or from an existing list.
fn collect_exr_paths(source: ProxySource) -> Vec<PathBuf> {
    let mut paths = match source {
        ProxySource::Directory(ref dir) => {
            let mut out = Vec::new();
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension()
                        .map(|e| {
                            e.to_str()
                                .map(|s| s.eq_ignore_ascii_case("exr"))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                    {
                        out.push(p);
                    }
                }
            }
            out
        }
        ProxySource::Files(ref list) => list
            .iter()
            .filter(|p| {
                p.extension()
                    .map(|e| {
                        e.to_str()
                            .map(|s| s.eq_ignore_ascii_case("exr"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .cloned()
            .collect(),
    };
    sort_exr_paths_by_file_name(&mut paths);
    paths
}

#[derive(Clone)]
enum ProxySource {
    Directory(PathBuf),
    Files(Vec<PathBuf>),
}

/// Default duration per still frame in the concat list (24 fps). Required for image/EXR inputs
/// or FFmpeg emits a near-zero duration stream and players only see a handful of frames.
const CONCAT_FRAME_DURATION_SECS: &str = "0.041666666666666664";

/// Write FFmpeg concat list (ffconcat version 1.0 + file + duration per EXR).
fn write_exr_concat_list(dir: &std::path::Path, exr_paths: &[PathBuf]) -> std::io::Result<PathBuf> {
    let list_path = dir.join(EXR_LIST_FILENAME);
    let mut content = String::new();
    content.push_str("ffconcat version 1.0\n");
    for p in exr_paths {
        let abs = p.canonicalize().unwrap_or_else(|_| p.clone());
        let s = abs.to_string_lossy().replace('\\', "/");
        content.push_str("file '");
        content.push_str(&s);
        content.push_str("'\n");
        content.push_str("duration ");
        content.push_str(CONCAT_FRAME_DURATION_SECS);
        content.push('\n');
    }
    // Concat demuxer ignores duration on the last segment; repeat last file so the prior duration applies.
    if let Some(last) = exr_paths.last() {
        let abs = last.canonicalize().unwrap_or_else(|_| last.clone());
        let s = abs.to_string_lossy().replace('\\', "/");
        content.push_str("file '");
        content.push_str(&s);
        content.push_str("'\n");
    }
    std::fs::write(&list_path, &content)?;
    Ok(list_path)
}

/// Run EXR sequence → single video proxy in a background thread.
/// Output: dst_dir/proxy.mkv — FFV1 lossless, 1080p height, keyframe every frame (-g 1).
/// Progress is updated by parsing FFmpeg stderr for "frame= N".
fn run_exr_to_video_proxy_in_background(
    source: ProxySource,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
) {
    if running.load(Ordering::Relaxed) {
        return;
    }

    let _ = std::fs::create_dir_all(&dst_dir);

    let exr_paths = collect_exr_paths(source.clone());
    let total = exr_paths.len();
    if total == 0 {
        running.store(false, Ordering::Relaxed);
        return;
    }

    let list_path = match write_exr_concat_list(&dst_dir, &exr_paths) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to write EXR list: {}", e);
            running.store(false, Ordering::Relaxed);
            return;
        }
    };

    let output_path = dst_dir.join(PROXY_VIDEO_FILENAME);

    running.store(true, Ordering::Relaxed);
    *progress.lock() = 0.0;

    thread::spawn(move || {
        // FFV1: lossless. -g 1: keyframe every frame. scale=-1:1080: 1080p height. -an: no audio.
        let mut child = match Command::new("ffmpeg")
            .arg("-y")
            .args(["-f", "concat"])
            .args(["-safe", "0"])
            .args(["-i", list_path.to_string_lossy().as_ref()])
            .args(["-vf", "scale=-1:1080"])
            .args(["-c:v", "ffv1"])
            .args(["-g", "1"])
            .args(["-level", "3"])
            .args(["-pix_fmt", "yuv420p"])
            .arg("-an")
            .arg(output_path.as_os_str())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to spawn ffmpeg: {}", e);
                running.store(false, Ordering::Relaxed);
                return;
            }
        };

        let stderr = child.stderr.take().expect("stderr piped");
        let reader = BufReader::new(stderr);
        // Parse lines like "frame=  123 fps=..." to update progress
        for line in reader.lines().flatten() {
            if let Some(frame_str) = line.split_whitespace().find(|s| s.starts_with("frame=")) {
                if let Some(num_str) = frame_str.strip_prefix("frame=") {
                    let num_str = num_str.trim();
                    if let Ok(n) = num_str.parse::<u64>() {
                        let p = (n as f32 / total as f32).min(1.0);
                        *progress.lock() = p;
                    }
                }
            }
        }

        let _ = child.wait();
        *progress.lock() = 1.0;
        running.store(false, Ordering::Relaxed);
    });
}

/// Start proxy generation from a directory (list EXR inside). Output: dst_dir/proxy.mkv.
pub fn run_from_directory_in_background(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
) {
    run_exr_to_video_proxy_in_background(
        ProxySource::Directory(src_dir),
        dst_dir,
        progress,
        running,
    );
}

/// Start proxy generation from a list of EXR file paths. Output: dst_dir/proxy.mkv.
pub fn run_from_files_in_background(
    exr_paths: Vec<PathBuf>,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
) {
    run_exr_to_video_proxy_in_background(ProxySource::Files(exr_paths), dst_dir, progress, running);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn exr_paths_sorted_by_file_name() {
        let mut paths = vec![
            PathBuf::from("/seq/frame_010.exr"),
            PathBuf::from("/seq/frame_2.exr"),
            PathBuf::from("/seq/frame_001.exr"),
        ];
        sort_exr_paths_by_file_name(&mut paths);
        assert!(
            paths[0].to_string_lossy().contains("001"),
            "expected lexicographic order by file name"
        );
        assert!(paths[1].to_string_lossy().contains("010"));
        assert!(paths[2].to_string_lossy().contains("2"));
    }
}
