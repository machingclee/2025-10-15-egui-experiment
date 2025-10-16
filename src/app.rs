use egui::WidgetText;

use crate::prisma;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct App {
    label: String,
    value: f32,
    folders: Vec<prisma::scripts_folder::Data>,
    selected_folder_id: i32,
    selected_scripts: Vec<prisma::shell_script::Data>,
    splitter_ratio: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            folders: vec![],
            selected_folder_id: -1,
            selected_scripts: vec![],
            splitter_ratio: 0.5,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::setup_custom_fonts(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                // egui::widgets::g lobal_theme_preference_buttons(ui);
            });
        });

        // Left panel for folders - starts small, can be resized
        egui::SidePanel::left("Folders Panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=600.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Scripts Folders");
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Folders to be shown ... WIP");
                });
            });

        // Central panel for scripts - takes remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(-6.0); // Reduce top padding
            ui.vertical_centered(|ui| {
                ui.label("Scripts");
            });
            ui.separator();

            // Example 1: Using Frame with uniform margin
            egui::Frame::new()
                .inner_margin(16.0) // Same margin on all sides
                .show(ui, |ui| {
                    ui.label("This is inside a Frame with 16px margin on all sides");
                });

            ui.add_space(10.0);

            // Example 2: Using group() - has default styling with background
            ui.group(|ui| {
                ui.label("This is inside a group() - has background and padding");
            });

            ui.add_space(10.0);

            // Example 3: Frame with background and stroke (most like a styled div)
            egui::Frame::new()
                .fill(ui.visuals().window_fill())
                .stroke(ui.visuals().window_stroke())
                .corner_radius(4.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.label("Frame with background, border, rounded corners, and 12px margin");
                });
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Scripts to be shown ... WIP");
            });
        });
    }
}
