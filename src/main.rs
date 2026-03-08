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

    let mut native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu, // Re-enable Wgpu since xcap is now used for screenshots
        viewport: egui::ViewportBuilder::default()
            .with_title("WPP Production Media Diferencial Player")
            .with_inner_size([1600.0, 900.0])
            .with_min_inner_size([900.0, 560.0]),
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
