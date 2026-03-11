// main.rs — DiffPlayerQC entry point

mod app;
mod decoder;
mod renderer;
mod types;
mod ui;

use eframe::{egui, CreationContext, App};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("=== DiffPlayerQC Startup ===");

    // Detect system dark/light mode
    let follow_dark = matches!(dark_light::detect(), dark_light::Mode::Dark);

    // Load app icon from embedded bytes (compiled into binary)
    let icon_data = {
        let icon_bytes = include_bytes!("../assets/Icon-iOS-Default-1024x1024@1x.png");
        match image::load_from_memory(icon_bytes) {
            Ok(img) => {
                let rgba = img.into_rgba8();
                let (w, h) = rgba.dimensions();
                let pixels = rgba.into_raw();
                Some(egui::IconData { rgba: pixels, width: w, height: h })
            }
            Err(e) => {
                log::warn!("Could not load app icon: {e}");
                None
            }
        }
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
        follow_system_theme: true,
        default_theme: if follow_dark { eframe::Theme::Dark } else { eframe::Theme::Light },
        centered: true,
        ..Default::default()
    };
    
    // --- MODIFICACIÓN: Inicializamos el sistema de audio ANTES de iniciar eframe ---
    log::info!("Inicializando sistema de audio CoreAudio/Rodio...");
    let (audio_stream, audio_handle) = match rodio::OutputStream::try_default() {
        Ok((s, h)) => (Some(s), Some(h)),
        Err(e) => {
            log::warn!("No se pudo inicializar el audio: {e}");
            (None, None)
        }
    };
    // -------------------------------------------------------------------------------

    log::info!("Starting eframe application loop...");
    eframe::run_native(
        "WPP Production Media Diferencial Player",
        native_options,
        // Usamos 'move' para que el closure sea dueño de audio_stream y audio_handle
        Box::new(move |cc: &CreationContext<'_>| {
            log::info!("CreationContext initialized, building app...");
            
            // Le pasamos el stream y el handle inicializados a DiffPlayerApp
            let app = app::DiffPlayerApp::new(cc, audio_stream, audio_handle);
            
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