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

    let mut native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: viewport_builder,
        follow_system_theme: true,
        default_theme: if follow_dark { eframe::Theme::Dark } else { eframe::Theme::Light },
        centered: true,
        ..Default::default()
    };
    // Use Vulkan, Metal, or GL backends explicitly to avoid unstable DX12 screenshot deadlocks on Windows
    native_options.wgpu_options.supported_backends = eframe::wgpu::Backends::VULKAN | eframe::wgpu::Backends::METAL | eframe::wgpu::Backends::GL;

    eframe::run_native(
        "WPP Production Media Diferencial Player",
        native_options,
        Box::new(|cc: &CreationContext<'_>| {
            let app = app::DiffPlayerApp::new(cc);
            Box::new(app) as Box<dyn App>
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(())
}
