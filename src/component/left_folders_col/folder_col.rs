use crate::component::left_folders_col::folder_item::FolderItem;
use crate::domain::folder::folder_command_handler::FolderCommand;
use crate::prisma::scripts_folder::Data;
use crate::{dispatch_folder_command, with_folder_state_reducer};
use eframe::emath::{Pos2, Rect};
use egui::epaint::RectShape;
use egui::{Color32, Frame, Id, Response, Stroke, Ui};
use prisma_client_rust::bigdecimal::ToPrimitive;
use std::sync::Arc;

pub struct FolderColumn;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
struct Location {
    row_index: i32,
}

impl FolderColumn {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, ctx: &egui::Context) {
        egui::SidePanel::left("Folders Panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(200.0..=600.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(format!("{}", "Scripts Folders"))
                        .strong()
                        .font(egui::FontId::proportional(16.0)),
                );
                ui.separator();
                ui.add_space(10.0);

                Self::add_folder_button(ui);

                ui.add_space(10.0);

                Self::folders(ui);
            });
    }

    fn folders(ui: &mut Ui) {
        // Remove stroke for all widget states

        let frame_without_stroke = Self::created_frame_without_stroke(ui);

        let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame_without_stroke, |ui| {
            egui::Frame::new()
                .fill(ui.visuals().panel_fill)
                .stroke(Stroke::NONE)
                .corner_radius(4.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Get direct access to state - we handle locking ourselves!
                        crate::with_folder_state(|state| {
                            let folders_vec = (*state.folder_list.read().unwrap()).clone();
                            let selected_id = *state.selected_folder_id.read().unwrap();
                            let rename_folder =
                                state.folder_to_rename.read().unwrap().as_ref().cloned();
                            let rename_text = state.rename_text.read().unwrap().as_ref().cloned();

                            let mut from: Option<Location> = None;
                            let mut to = None;

                            if folders_vec.is_empty() {
                                ui.label("No folders yet...");
                            } else {
                                for (row_idx, folder) in (&*folders_vec).into_iter().enumerate() {
                                    let (item_location, response) = Self::render_dnd_folder_item(
                                        ui,
                                        selected_id,
                                        &rename_folder,
                                        &rename_text,
                                        row_idx,
                                        &folder,
                                    );

                                    if let (Some(pointer), Some(hovered_payload)) = (
                                        ui.input(|i| i.pointer.interact_pos()),
                                        response.dnd_hover_payload::<Location>(),
                                    ) {
                                        Self::handle_drop_event(
                                            ui,
                                            from,
                                            to,
                                            row_idx,
                                            item_location,
                                            response,
                                            pointer,
                                            hovered_payload,
                                        );
                                    }
                                }
                            }
                        });
                    });
                });
        });
    }

    fn created_frame_without_stroke(ui: &mut Ui) -> Frame {
        ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke::NONE;
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::NONE;
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::NONE;
        ui.visuals_mut().widgets.active.bg_stroke = Stroke::NONE;

        let frame = egui::Frame::new()
            .fill(ui.visuals().panel_fill)
            .stroke(Stroke::NONE)
            .corner_radius(4.0)
            .inner_margin(0.0);

        ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::WHITE;
        frame
    }

    fn handle_drop_event(
        ui: &mut Ui,
        mut from: Option<Location>,
        mut to: Option<Location>,
        row_idx: usize,
        item_location: Location,
        response: Response,
        pointer: Pos2,
        hovered_payload: Arc<Location>,
    ) {
        let rect = response.rect;

        // Preview insertion:
        let stroke = egui::Stroke::new(2.0, Color32::from_black_alpha(60));
        let insert_row_idx = if *hovered_payload == item_location {
            // We are dragged onto ourselves
            ui.painter().hline(rect.x_range(), rect.center().y, stroke);
            row_idx
        } else if pointer.y < rect.center().y {
            // Above us
            ui.painter().hline(rect.x_range(), rect.top() - 2.0, stroke);
            row_idx
        } else {
            // Below us
            ui.painter()
                .hline(rect.x_range(), rect.bottom() + 2.0, stroke);
            row_idx + 1
        };

        let dragged_payload_opt = response.dnd_release_payload::<Location>();
        if let Some(dragged_payload) = dragged_payload_opt {
            // The user dropped onto this item.
            from = Some(*dragged_payload);
            to = Some(Location {
                row_index: insert_row_idx.to_i32().unwrap(),
            });

            let from_row_index = dragged_payload.row_index;

            let moving_downwards = insert_row_idx as i32 >= from_row_index;
            // If moving downwards, we need to adjust the target index since the item will be removed first
            let to_row_index = if moving_downwards {
                (insert_row_idx as i32 - 1).max(0)
            } else {
                insert_row_idx as i32
            };

            dispatch_folder_command(FolderCommand::ReorderFolders {
                from_index: from_row_index,
                to_index: to_row_index,
            });
        }
    }

    fn render_dnd_folder_item(
        ui: &mut Ui,
        selected_id: Option<i32>,
        rename_folder: &Option<Arc<Data>>,
        rename_text: &Option<String>,
        row_idx: usize,
        folder: &&Data,
    ) -> (Location, Response) {
        let item_id = Id::new(("my_drag_and_drop_demo", row_idx));
        let item_location = Location {
            row_index: row_idx.to_i32().unwrap(),
        };
        let inner_response = ui.horizontal(|ui| {
            let dnd_res = ui.dnd_drag_source(item_id, item_location, |ui| {
                ui.label(egui::RichText::new(" :: "));
            });
            Self::render_folder(
                ui,
                selected_id,
                rename_folder.clone(),
                &rename_text,
                &folder,
            );
            (dnd_res, ())
        });

        ui.add_space(2.0);

        let response = inner_response.response;
        (item_location, response)
    }

    fn render_folder(
        ui: &mut Ui,
        selected_id: Option<i32>,
        rename_folder: Option<Arc<Data>>,
        rename_text: &Option<String>,
        folder: &&Data,
    ) {
        let is_renaming = rename_folder
            .as_ref()
            .map(|f| f.id == folder.id)
            .unwrap_or(false);
        let display_name = if is_renaming {
            rename_text.as_ref().unwrap_or(&folder.name)
        } else {
            &folder.name
        };
        let mut folder_item = FolderItem::new(folder, selected_id, display_name);
        folder_item.view(ui);
    }

    pub fn add_folder_button(ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            let response =
                ui.button(egui::RichText::new("Add Folder").font(egui::FontId::proportional(18.0)));
            if response.clicked() {
                dispatch_folder_command(FolderCommand::CreateFolder {});
            }
        });
    }
}
