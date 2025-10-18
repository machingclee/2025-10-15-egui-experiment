// Pure view function - only handles rendering
fn render_scripts_header(
    ui: &mut egui::Ui,
    selected_folder: Option<&crate::prisma::scripts_folder::Data>,
) {
    let display_name = selected_folder
        .map(|f| f.name.as_str())
        .unwrap_or("No folder selected");
    ui.label(format!("Scripts ({})", display_name));
    ui.separator();
}

// Approach 2: Callback pattern - pass view logic to data access
fn with_selected_folder<F, R>(state: &'static crate::state::folder_state::FoldersState, f: F) -> R
where
    F: FnOnce(Option<&crate::prisma::scripts_folder::Data>) -> R,
{
    let folder_id = *state.selected_folder_id.read().unwrap();
    let folders = state.folder_list.read().unwrap();
    let selected_folder = folders.iter().find(|f| Some(f.id) == folder_id);
    f(selected_folder)
}

pub fn scripts_col(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(-6.0);

        let state = crate::get_folder_state_ref();
        with_selected_folder(state, |selected_folder| {
            render_scripts_header(ui, selected_folder);
        });

        // Example 1: Using Frame with uniform margin
        egui::Frame::new()
            .inner_margin(16.0) // Same margin on all sides
            .show(ui, |ui| {
                ui.label("This is inside a Frame with 16px margin on all sides");
            });

        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("Scripts to be shown ... WIP");
        });
    });
}
