#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let rt = tokio::runtime::Runtime::new().unwrap();
    shell_script_manager::RT_HANDLE
        .set(rt.handle().clone())
        .unwrap();

    // Initialize global Prisma client
    rt.block_on(async {
        shell_script_manager::PRISMA_CLIENT
            .set(shell_script_manager::prisma::new_client().await.unwrap())
            .unwrap();
    });

    // Initialize event system
    let (tx, rx) = crossbeam::channel::unbounded();
    shell_script_manager::EVENT_SENDER.set(tx).unwrap();
    shell_script_manager::EVENT_RECEIVER.set(rx).unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 400.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    // Spawn a task to keep the runtime alive
    std::thread::spawn(move || {
        rt.block_on(async {
            // Keep alive until signal
            let _ = tokio::signal::ctrl_c().await;
        });
    });

    eframe::run_native(
        "Shell Script Managers",
        native_options,
        Box::new(|cc| Ok(Box::new(shell_script_manager::App::new(cc)))),
    )
}
