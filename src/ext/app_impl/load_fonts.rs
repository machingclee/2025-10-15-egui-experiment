use crate::App;

impl App {
    pub fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Try to load system fonts
        #[cfg(target_os = "macos")]
        {
            if let Some(font_data) = Self::load_system_font("Helvetica Neue") {
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_font".to_owned());
            }

            if let Some(font_data) = Self::load_system_font("Menlo") {
                fonts.font_data.insert(
                    "system_mono".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "system_mono".to_owned());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(font_data) = Self::load_system_font("Segoe UI") {
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_font".to_owned());
            }

            if let Some(font_data) = Self::load_system_font("Consolas") {
                fonts.font_data.insert(
                    "system_mono".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "system_mono".to_owned());
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(font_data) = Self::load_system_font("Ubuntu") {
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_font".to_owned());
            }

            if let Some(font_data) = Self::load_system_font("Ubuntu Mono") {
                fonts.font_data.insert(
                    "system_mono".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "system_mono".to_owned());
            }
        }

        ctx.set_fonts(fonts);
    }

    fn load_system_font(name: &str) -> Option<Vec<u8>> {
        use font_loader::system_fonts;

        let property = system_fonts::FontPropertyBuilder::new()
            .family(name)
            .build();

        match system_fonts::get(&property) {
            Some((font_data, _)) => Some(font_data),
            None => None,
        }
    }
}
