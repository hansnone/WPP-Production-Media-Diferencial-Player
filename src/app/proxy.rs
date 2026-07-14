use super::{proxy_bridge, DiffPlayerApp};
use crate::types::Channel;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

impl DiffPlayerApp {
    /// Start EXR→PNG proxy generation from a directory (lists .exr inside). When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_dir(
        &mut self,
        src_dir: PathBuf,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() {
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        crate::proxy::run_from_directory_in_background(
            src_dir,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
            Arc::clone(&self.proxy_error),
        );
    }

    /// Start EXR→PNG proxy generation from a list of EXR file paths. When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_files(
        &mut self,
        exr_paths: Vec<PathBuf>,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() || exr_paths.is_empty() {
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        crate::proxy::run_from_files_in_background(
            exr_paths,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
            Arc::clone(&self.proxy_error),
        );
    }

    /// True if proxy generation is currently running.
    pub fn proxy_running(&self) -> bool {
        self.proxy_running.load(Ordering::Relaxed)
    }

    /// Current proxy progress 0.0..=1.0.
    pub fn proxy_progress(&self) -> f32 {
        *self.proxy_progress.lock()
    }

    pub(super) fn complete_proxy_if_ready(&mut self, ctx: &egui::Context) {
        if self.proxy_running()
            || self.proxy_target_channel.is_none()
            || self.proxy_temp_dir.is_none()
        {
            return;
        }
        let dir = self.proxy_temp_dir.take().unwrap();
        let channel = self.proxy_target_channel.take().unwrap();
        let proxy_video = proxy_bridge::proxy_video_path(&dir);
        if proxy_video.exists() {
            self.proxy_temp_dirs.push(dir);
            let path_str = proxy_video.to_string_lossy().to_string();
            self.open_video_from_path(path_str, channel, ctx);
        }
    }
}
