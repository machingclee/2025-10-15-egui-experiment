pub struct ScriptsColumn {}

impl ScriptsColumn {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(-6.0);

            let state = crate::get_folder_state_ref();
            with_selected_folder(state, |selected_folder| {
                render_scripts_header(ui, selected_folder);
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Scripts to be shown ... WIP");
            });
        });
    }
}

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
