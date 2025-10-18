#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    // Choose database location based on build mode
    let db_path = if cfg!(debug_assertions) {
        // In debug mode, use current directory for easier development
        std::env::current_dir().unwrap().join("database.db")
    } else {
        // In release mode, use proper app data directory
        let app_data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("ShellScriptManager");

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&app_data_dir).ok();

        app_data_dir.join("database.db")
    };

    let db_url = format!("file:{}", db_path.display());

    let rt = tokio::runtime::Runtime::new().unwrap();
    shell_script_manager::RT_HANDLE
        .set(rt.handle().clone())
        .unwrap();

    rt.block_on(async {
        match shell_script_manager::prisma::new_client_with_url(&db_url).await {
            Ok(client) => {
                // Initialize database schema automatically for desktop app
                if let Err(e) = shell_script_manager::db::get_db::initialize_database(&client).await
                {
                    eprintln!("Failed to initialize database: {}", e);
                    eprintln!("Please check database permissions or file path");
                    std::process::exit(1);
                }

                shell_script_manager::PRISMA_CLIENT.set(client).unwrap();
                println!("Database connection established successfully");
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                eprintln!("Please ensure the database exists by running: npm run migrate:dev");
                eprintln!(
                    "If deploying to production, run migrations as part of your deployment process."
                );
                std::process::exit(1);
            }
        }
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
