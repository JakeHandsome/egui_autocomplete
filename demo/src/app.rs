use std::collections::BTreeSet;

use eframe::egui;
use egui::{TextEdit, Ui, Vec2};
use egui_autocomplete::AutoCompleteTextEdit;

const STARTER_LIST: &str = r#"writer
seat
dog
worker
grade
face
ahead
immediately
dance
too
equipment
alike
noun
soil
floor
both
copper
tune
say
plural
bark
exciting
hold
price
coat
creature
news
across
strange
mud
force
her
told
hour
natural
finally
dot
mysterious
rapidly
larger
race
treated
"#;

pub struct TemplateApp {
    auto_complete1: AutoCompleteExample,
    auto_complete2: AutoCompleteExample,
}

struct AutoCompleteExample {
    multi_input: String,
    search_field: String,
}

impl AutoCompleteExample {
    fn update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        let inputs: Vec<String> = self
            .multi_input
            .lines()
            .map(|x| x.to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        ui.add(AutoCompleteTextEdit::new(
            &mut self.search_field,
            &inputs,
            10,
        ));
        ui.add(TextEdit::multiline(&mut self.multi_input));
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            auto_complete1: AutoCompleteExample {
                multi_input: STARTER_LIST.to_string(),
                search_field: Default::default(),
            },
            auto_complete2: AutoCompleteExample {
                multi_input: Default::default(),
                search_field: Default::default(),
            },
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("egui_autocomplete demo");
                ui.label(
                    r#"Enter text in the single line entry for auto_complete.
Add new lines in the multiline textbox to add to the autocomplete menu.
Use arrow keys to select completion.
Use enter, tab or mouseclick to apply completion."#,
                );
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    ui.available_size() / Vec2::new(2., 1.),
                    egui::Layout::top_down(egui::Align::Max),
                    |ui| {
                        self.auto_complete1.update(ctx, ui);
                    },
                );
                ui.separator();
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.auto_complete2.update(ctx, ui);
                    },
                );
            });
        });
    }
}
