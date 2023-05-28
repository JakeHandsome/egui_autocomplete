use std::collections::{BTreeSet};

use eframe::egui;
use egui::{TextEdit, Ui};
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
      ui.vertical(|ui| {
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
      });
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
         ui.heading("My egui Application");
         ui.separator();
         ui.horizontal(|ui| {
            self.auto_complete1.update(ctx, ui);
            ui.separator();
            self.auto_complete2.update(ctx, ui);
         });
      });
   }
}
