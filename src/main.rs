//! Punto de entrada del binario: logging, icono de ventana y arranque de `eframe`.
//!
//! Los módulos de la app viven en `app/`, `decoder`, `renderer`, etc. No contiene lógica de QC;
//! solo configura el entorno nativo antes de delegar en [`DiffPlayerApp`](crate::app::DiffPlayerApp).

mod app;
mod decoder;
mod error;
pub use error::AppError;
pub mod metrics;
mod proxy;
mod renderer;
mod trace_log;
pub mod thumbnail;
mod types;
mod ui;

use eframe::{egui, App, CreationContext};
use image::imageops::FilterType;

fn main() -> anyhow::Result<()> {
    // Escupir logs a un fichero temporal incondicionalmente para poder leer por qué no arranca la vista
    let log_file = std::fs::File::create("/tmp/diffplayerqc_app.log")?;
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Pipe(Box::new(log_file)));
    builder.filter_level(log::LevelFilter::Info);
    builder.init();

    log::info!("=== DiffPlayerQC Startup (LOG REDIRECTED) ===");

    // Human-readable trace log (one file per run: yyyy_mm_dd_hh_mm_ss_Diff_start.log)
    let log_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    if let Err(e) = trace_log::init(log_dir) {
        log::warn!("Trace log init failed: {e}");
    } else {
        trace_log::log("DiffPlayerQC started");
    }

    // Load icon and resize to 64x64 so macOS window creation doesn't block (large icons can hang).
    let icon_data: Option<egui::IconData> = {
        let icon_bytes = include_bytes!("../assets/Icon-iOS-Default-1024x1024@1x.png");
        image::load_from_memory(icon_bytes).ok().map(|img| {
            let rgba = img.into_rgba8();
            let small = image::imageops::resize(&rgba, 64, 64, FilterType::Triangle);
            let (w, h) = small.dimensions();
            let pixels = small.into_raw();
            egui::IconData {
                rgba: pixels,
                width: w,
                height: h,
            }
        })
    };

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_title("WPP Production Media Diferencial Player")
        .with_inner_size([1600.0, 900.0])
        .with_min_inner_size([900.0, 560.0]);

    if let Some(icon) = icon_data {
        viewport_builder = viewport_builder.with_icon(std::sync::Arc::new(icon));
    }

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: viewport_builder,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        centered: false,
        ..Default::default()
    };

    log::info!("Starting eframe application loop...");

    eframe::run_native(
        "WPP Production Media Diferencial Player",
        native_options,
        Box::new(|cc: &CreationContext<'_>| {
            log::info!("CreationContext initialized, building app...");
            trace_log::log("CreationContext ready, building app");
            let app = app::DiffPlayerApp::new(cc);
            Box::new(app) as Box<dyn App>
        }),
    )
    .map_err(|e| {
        log::error!("Eframe execution error: {e}");
        anyhow::anyhow!("{e}")
    })?;

    log::info!("Application exited cleanly.");
    Ok(())
}
