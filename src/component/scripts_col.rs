pub fn scripts_col(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(-6.0); // Reduce top padding
        ui.label("Scripts");
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
