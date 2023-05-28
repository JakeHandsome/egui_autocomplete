use egui::{Context, Id, Key, Modifiers, Widget, WidgetWithState};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::{min, Reverse};

#[derive(Clone, Default)]
pub struct AutoCompleteTextEditState {
   selected_index: Option<usize>,
}

impl AutoCompleteTextEditState {
   pub fn load(ctx: &Context, id: Id) -> Option<Self> {
      ctx.data_mut(|d| d.get_persisted(id))
   }

   pub fn store(self, ctx: &Context, id: Id) {
      ctx.data_mut(|d| d.insert_persisted(id, self));
   }

   fn update_index(
      &mut self,
      down_pressed: bool,
      up_pressed: bool,
      match_results_count: usize,
      max_suggestions: usize,
   ) {
      self.selected_index = match self.selected_index {
         // Increment selected index when down is pressed, limit it to the number of matches
         Some(index) if down_pressed => {
            if index + 1 < min(match_results_count, max_suggestions) {
               Some(index + 1)
            } else {
               Some(index)
            }
         }
         // Decrement selected index if up is pressed. Deselect if at first index
         Some(index) if up_pressed => {
            if index == 0 {
               None
            } else {
               Some(index - 1)
            }
         }
         // If nothing is selected and down is pressed, select first item
         None if down_pressed => Some(0),
         // Do nothing if no keys are pressed
         Some(index) => Some(index),
         None => None,
      }
   }
}

pub struct AutoCompleteTextEdit<'a> {
   text_field: &'a mut String,
   search: &'a [String],
   max_suggestions: usize,
}

impl<'a> WidgetWithState for AutoCompleteTextEdit<'a> {
   type State = AutoCompleteTextEditState;
}

impl<'a> Widget for AutoCompleteTextEdit<'a> {
   fn ui(self, ui: &mut egui::Ui) -> egui::Response {
      let Self {
         text_field,
         search,
         max_suggestions,
      } = self;
      let text_response = ui.text_edit_singleline(text_field);
      let up_pressed = text_response.has_focus()
         && ui.input_mut(|input| input.consume_key(Modifiers::default(), Key::ArrowUp));
      let down_pressed = text_response.has_focus()
         && ui.input_mut(|input| input.consume_key(Modifiers::default(), Key::ArrowDown));
      let id = ui.make_persistent_id(text_response.id);
      let mut state = AutoCompleteTextEditState::load(ui.ctx(), id).unwrap_or_default();

      let matcher = SkimMatcherV2::default().ignore_case();

      let mut match_results = search
         .iter()
         .filter_map(|s| {
            let score = matcher.fuzzy_match(s, text_field);
            score.map(|x| (s, x))
         })
         .collect::<Vec<_>>();
      match_results.sort_by_key(|k| Reverse(k.1));

      if text_response.changed()
         || (state.selected_index.is_some() && state.selected_index.unwrap() >= match_results.len())
      {
         state.selected_index = None;
      }

      state.update_index(
         down_pressed,
         up_pressed,
         match_results.len(),
         max_suggestions,
      );

      let enter_pressed = ui.input_mut(|input| input.key_pressed(Key::Enter));
      if let (Some(index), true) = (state.selected_index, enter_pressed) {
         *text_field = match_results[index].0.clone();
      }
      egui::popup::popup_below_widget(ui, id, &text_response, |ui| {
         ui.set_min_width(200.0); // if you want to control the size
         for (i, (output, _)) in match_results.iter().take(max_suggestions).enumerate() {
            let mut highlighed = if let Some(x) = state.selected_index {
               x == i
            } else {
               false
            };
            if ui.toggle_value(&mut highlighed, *output).clicked() {
               *text_field = (*output).clone();
            }
         }
      });
      if !text_field.is_empty() && text_response.has_focus() && !match_results.is_empty() {
         ui.memory_mut(|mem| mem.open_popup(id));
      } else {
         ui.memory_mut(|mem| {
            if mem.is_popup_open(id) {
               mem.close_popup()
            }
         });
      }

      state.store(ui.ctx(), id);

      text_response
   }
}

impl<'a> AutoCompleteTextEdit<'a> {
   pub fn new(text_field: &'a mut String, search: &'a [String], max_suggestions: usize) -> Self {
      Self {
         text_field,
         search,
         max_suggestions,
      }
   }
}
