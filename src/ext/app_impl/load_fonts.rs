use crate::App;

impl App {
    pub fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Try to load system fonts with all weights and language support
        #[cfg(target_os = "macos")]
        {
            let mut font_index = 0;
            
            // Load Helvetica Neue with different weights
            let helvetica_weights = vec![
                ("Helvetica Neue", "helvetica_regular"),
                ("Helvetica Neue Bold", "helvetica_bold"),
                ("Helvetica Neue Medium", "helvetica_medium"),
                ("Helvetica Neue Light", "helvetica_light"),
            ];
            
            for (font_name, font_key) in helvetica_weights {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(font_index, font_key.to_owned());
                    font_index += 1;
                }
            }

            // Load Asian language fonts for comprehensive language support
            let cjk_fonts = vec![
                ("PingFang SC", "pingfang_sc"),        // Simplified Chinese
                ("PingFang TC", "pingfang_tc"),        // Traditional Chinese
                ("Hiragino Sans", "hiragino"),         // Japanese
                ("Apple SD Gothic Neo", "apple_gothic"), // Korean
            ];
            
            for (font_name, font_key) in cjk_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .push(font_key.to_owned());
                }
            }

            // Load emoji and symbol fonts
            if let Some(font_data) = Self::load_system_font("Apple Color Emoji") {
                fonts.font_data.insert(
                    "emoji".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .push("emoji".to_owned());
            }

            // Load monospace fonts with different weights
            let mono_fonts = vec![
                ("Menlo", "menlo_regular"),
                ("Menlo Bold", "menlo_bold"),
                ("SF Mono", "sf_mono"),
            ];
            
            for (font_name, font_key) in mono_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Monospace)
                        .unwrap()
                        .push(font_key.to_owned());
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Load Segoe UI with different weights
            let segoe_weights = vec![
                ("Segoe UI", "segoe_regular"),
                ("Segoe UI Bold", "segoe_bold"),
                ("Segoe UI Semibold", "segoe_semibold"),
                ("Segoe UI Light", "segoe_light"),
            ];
            
            let mut font_index = 0;
            for (font_name, font_key) in segoe_weights {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(font_index, font_key.to_owned());
                    font_index += 1;
                }
            }

            // Load CJK fonts for Windows
            let cjk_fonts = vec![
                ("Microsoft YaHei", "yahei"),          // Simplified Chinese
                ("Microsoft JhengHei", "jhenghei"),    // Traditional Chinese
                ("Yu Gothic", "yugothic"),             // Japanese
                ("Malgun Gothic", "malgun"),           // Korean
            ];
            
            for (font_name, font_key) in cjk_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .push(font_key.to_owned());
                }
            }

            // Load emoji support
            if let Some(font_data) = Self::load_system_font("Segoe UI Emoji") {
                fonts.font_data.insert(
                    "emoji".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .push("emoji".to_owned());
            }

            // Load monospace fonts
            let mono_fonts = vec![
                ("Consolas", "consolas"),
                ("Consolas Bold", "consolas_bold"),
                ("Cascadia Code", "cascadia"),
            ];
            
            for (font_name, font_key) in mono_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Monospace)
                        .unwrap()
                        .push(font_key.to_owned());
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Load Ubuntu/DejaVu fonts with different weights
            let linux_fonts = vec![
                ("Ubuntu", "ubuntu_regular"),
                ("Ubuntu Bold", "ubuntu_bold"),
                ("Ubuntu Medium", "ubuntu_medium"),
                ("Ubuntu Light", "ubuntu_light"),
                ("DejaVu Sans", "dejavu_sans"),
            ];
            
            let mut font_index = 0;
            for (font_name, font_key) in linux_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(font_index, font_key.to_owned());
                    font_index += 1;
                }
            }

            // Load CJK fonts for Linux
            let cjk_fonts = vec![
                ("Noto Sans CJK SC", "noto_sc"),       // Simplified Chinese
                ("Noto Sans CJK TC", "noto_tc"),       // Traditional Chinese
                ("Noto Sans CJK JP", "noto_jp"),       // Japanese
                ("Noto Sans CJK KR", "noto_kr"),       // Korean
            ];
            
            for (font_name, font_key) in cjk_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .push(font_key.to_owned());
                }
            }

            // Load emoji support
            if let Some(font_data) = Self::load_system_font("Noto Color Emoji") {
                fonts.font_data.insert(
                    "emoji".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .push("emoji".to_owned());
            }

            // Load monospace fonts
            let mono_fonts = vec![
                ("Ubuntu Mono", "ubuntu_mono"),
                ("Ubuntu Mono Bold", "ubuntu_mono_bold"),
                ("DejaVu Sans Mono", "dejavu_mono"),
            ];
            
            for (font_name, font_key) in mono_fonts {
                if let Some(font_data) = Self::load_system_font(font_name) {
                    fonts.font_data.insert(
                        font_key.to_owned(),
                        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Monospace)
                        .unwrap()
                        .push(font_key.to_owned());
                }
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
