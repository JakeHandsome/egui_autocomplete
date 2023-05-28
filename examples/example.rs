use std::collections::BTreeSet;

use eframe::egui;
use egui_autocomplete::AutoCompleteTextEdit;

fn main() -> Result<(), eframe::Error> {
   let options = eframe::NativeOptions {
      ..Default::default()
   };
   eframe::run_native(
      "My egui App",
      options,
      Box::new(|_cc| Box::<MyApp>::default()),
   )
}

struct MyApp {
   text_field: String,
   search_field: String,
   search_field2: String,
   inputs: BTreeSet<String>,
}

impl Default for MyApp {
   fn default() -> Self {
      Self {
         text_field: Default::default(),
         search_field: Default::default(),
         search_field2: Default::default(),
         inputs: Default::default(),
      }
   }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
         ui.heading("My egui Application");
         ui.horizontal(|ui| {
            let response = ui.text_edit_singleline(&mut self.text_field);
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
               || ui.button("Add").clicked()
            {
               self.inputs.insert(self.text_field.clone());
               self.text_field = "".into();
            }
         });
         ui.separator();
         ui.horizontal(|ui| {
            ui.add(AutoCompleteTextEdit::new(
               &mut self.search_field,
               &self.inputs.clone().into_iter().collect::<Vec<_>>(),
               10,
            ));

            let list2 = vec![
               "writer",
               "seat",
               "dog",
               "worker",
               "grade",
               "face",
               "ahead",
               "immediately",
               "dance",
               "too",
               "equipment",
               "alike",
               "noun",
               "soil",
               "floor",
               "both",
               "copper",
               "tune",
               "say",
               "plural",
               "bark",
               "exciting",
               "hold",
               "price",
               "coat",
               "creature",
               "news",
               "across",
               "strange",
               "mud",
               "force",
               "her",
               "told",
               "hour",
               "natural",
               "finally",
               "dot",
               "mysterious",
               "rapidly",
               "larger",
               "race",
               "treated",
            ]
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
            ui.add(AutoCompleteTextEdit::new(
               &mut self.search_field2,
               &list2,
               10,
            ));
         });

         ui.columns(2, |columns| {
            columns[0].label("Input text");
            for input in &self.inputs {
               columns[0].label(input);
            }
            columns[1].label("Search Results");
         });
      });
   }
}
