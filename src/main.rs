use eframe::egui;

// Constants
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_MIN_WIDTH: f32 = 120.0;
const WINDOW_MIN_HEIGHT: f32 = 100.0;
const DEFAULT_GRID_SIZE: i32 = 5;

/// Entry Point for basic sample settings editor
/// Launches native SettingsUi app
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        //.with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "egui testing",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(SettingsUi::default()))
        }),
    )
}

/// Enum for radial buttons, placeholder options
#[derive(PartialEq, Debug)]
enum RadioStation {
    FirstStation,
    SecondStation,
    ThirdStation,
}

/// Structure for the UI of a simple settings display
struct SettingsUi {
    grid_size: i32,
    grid_size_str: String,
    radio: RadioStation,
    check_one: bool,
    check_two: bool,
    show_editor: bool,
}

impl Default for SettingsUi {
    fn default() -> Self {
        Self {
            grid_size: DEFAULT_GRID_SIZE,
            grid_size_str: DEFAULT_GRID_SIZE.to_string(),
            radio: RadioStation::FirstStation,
            check_one: false,
            check_two: false,
            show_editor: false,
        }
    }
}
impl SettingsUi {
    /// Simple check on string parsing, reverts to default on error
    fn update_grid_size(&mut self) {
        match self.grid_size_str.trim().parse() {
            Ok(num) => {
                if num > 0 {
                    self.grid_size = num;
                } else {
                    self.grid_size_str = self.grid_size.to_string();
                }
            }
            Err(_err) => {
                self.grid_size_str = self.grid_size.to_string();
            }
        }
    }
}

impl eframe::App for SettingsUi {
    /// Main Application Loop consisting of two sections:
    ///     Central Panel: Displays settings values
    ///     Popup Window: Allows immediate editing of settings
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Settings Display");
            ui.horizontal(|ui| {
                ui.label(format!("Grid Size: {}", self.grid_size));
            });
            ui.horizontal(|ui| {
                ui.label("First Setting: ");
                ui.label(if self.check_one { "True" } else { "False" });
            });
            ui.horizontal(|ui| {
                ui.label("Second Setting: ");
                ui.label(if self.check_two { "True" } else { "False" });
            });
            ui.horizontal(|ui| {
                ui.label("Radial Choice: ");
                // Placeholder values (Get it? Radial? Radio?)
                ui.label(match self.radio {
                    RadioStation::FirstStation => "99.3 FM",
                    RadioStation::SecondStation => "880 AM",
                    RadioStation::ThirdStation => "102.5 FM",
                });
            });
            if ui.button("Edit Settings").clicked() {
                self.show_editor = true;
            }
        });
        if self.show_editor {
            egui::Window::new("Settings")
                .default_pos([200.0, 200.0])
                .title_bar(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Grid Size: ");
                        ui.text_edit_singleline(&mut self.grid_size_str);
                        if ui.button("Apply").clicked() {
                            self.update_grid_size();
                        }
                    });
                    ui.checkbox(&mut self.check_one, "Check me out!");
                    ui.checkbox(&mut self.check_two, "Check, please!");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.radio, RadioStation::FirstStation, "99.3 FM");
                        ui.radio_value(&mut self.radio, RadioStation::SecondStation, "880 AM");
                        ui.radio_value(&mut self.radio, RadioStation::ThirdStation, "102.5 FM");
                    });
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_editor = false;
                    }
                    ui.collapsing("Disclaimer", |ui| {
                        ui.label("I've nothing to disclaim,\nfor I have done nothing wrong.");
                    });
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_settings() {
        let default: SettingsUi = SettingsUi::default();
        assert_eq!(default.grid_size, DEFAULT_GRID_SIZE);
        assert_eq!(default.check_one, false);
        assert_eq!(default.check_two, false);
        assert_eq!(default.radio, RadioStation::FirstStation);
        assert_eq!(default.show_editor, false);
    }

    // Tests for proper validation when invalid grid size string is input
    /// Invalid Grid Size Case 1: Non-Numeric Input
    #[test]
    fn test_non_numeric_grid_size_string() {
        let mut settings: SettingsUi = SettingsUi::default();
        settings.grid_size_str = String::from("BadTest");
        settings.update_grid_size();
        assert_eq!(settings.grid_size, DEFAULT_GRID_SIZE);
        assert_eq!(settings.grid_size_str, DEFAULT_GRID_SIZE.to_string());
    }
    /// Invalid Grid Size Case 2: Non-Positive Input
    #[test]
    fn test_non_positive_grid_size_string() {
        let mut settings: SettingsUi = SettingsUi::default();
        settings.grid_size_str = String::from("0");
        settings.update_grid_size();
        assert_eq!(settings.grid_size, DEFAULT_GRID_SIZE);
        assert_eq!(settings.grid_size_str, DEFAULT_GRID_SIZE.to_string());
    }
}
