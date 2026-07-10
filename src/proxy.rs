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

use crate::error::AppError;

pub fn validate_ffmpeg_binary() -> Result<PathBuf, AppError> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::FfmpegNotFound
            } else {
                AppError::Io(e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::FfmpegCommandFailed {
            status: output.status.code(),
            stderr,
        });
    }

    Ok(PathBuf::from("ffmpeg"))
}

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
    error: Arc<Mutex<Option<String>>>,
) {
    if running.load(Ordering::Relaxed) {
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&dst_dir) {
        log::warn!("Failed to create proxy dir {:?}: {}", dst_dir, e);
    }

    let exr_paths = collect_exr_paths(source.clone());
    let total = exr_paths.len();
    if total == 0 {
        running.store(false, Ordering::Relaxed);
        return;
    }

    let list_path = match write_exr_concat_list(&dst_dir, &exr_paths) {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("Failed to write EXR list: {}", e);
            log::error!("{}", msg);
            *error.lock() = Some(msg);
            running.store(false, Ordering::Relaxed);
            return;
        }
    };

    let output_path = dst_dir.join(PROXY_VIDEO_FILENAME);

    running.store(true, Ordering::Relaxed);
    *progress.lock() = 0.0;

    const FFMPEG_SCALE: &str = "scale=-1:1080";
    const FFMPEG_GOP: &str = "1";
    const FFMPEG_LEVEL: &str = "3";
    const FFMPEG_PIX_FMT: &str = "yuv420p";

    thread::spawn(move || {
        // FFV1: lossless. -g 1: keyframe every frame. scale=-1:1080: 1080p height. -an: no audio.
        let mut child = match Command::new("ffmpeg")
            .arg("-y")
            .args(["-f", "concat"])
            .args(["-safe", "0"])
            .args(["-i", list_path.to_string_lossy().as_ref()])
            .args(["-vf", FFMPEG_SCALE])
            .args(["-c:v", "ffv1"])
            .args(["-g", FFMPEG_GOP])
            .args(["-level", FFMPEG_LEVEL])
            .args(["-pix_fmt", FFMPEG_PIX_FMT])
            .arg("-an")
            .arg(output_path.as_os_str())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Failed to spawn ffmpeg: {}", e);
                log::error!("{}", msg);
                *error.lock() = Some(msg);
                running.store(false, Ordering::Relaxed);
                return;
            }
        };

        let Some(stderr) = child.stderr.take() else {
            let msg = "ffmpeg stderr was not piped as expected".to_string();
            log::error!("{}", msg);
            *error.lock() = Some(msg);
            running.store(false, Ordering::Relaxed);
            return;
        };
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

        match child.wait() {
            Ok(status) if !status.success() => {
                let msg = format!("FFmpeg failed with status: {}", status);
                log::warn!("{}", msg);
                *error.lock() = Some(msg);
            }
            Err(e) => {
                let msg = format!("Failed to wait on ffmpeg child process: {}", e);
                log::warn!("{}", msg);
                *error.lock() = Some(msg);
            }
            _ => {}
        }
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
    error: Arc<Mutex<Option<String>>>,
) {
    run_exr_to_video_proxy_in_background(
        ProxySource::Directory(src_dir),
        dst_dir,
        progress,
        running,
        error,
    );
}

/// Start proxy generation from an explicit list of files. Output: dst_dir/proxy.mkv.
pub fn run_from_files_in_background(
    exr_paths: Vec<PathBuf>,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
) {
    run_exr_to_video_proxy_in_background(
        ProxySource::Files(exr_paths),
        dst_dir,
        progress,
        running,
        error,
    );
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
