#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc=include_str!("../README.md")]
//! # Example
//! ```rust
//! use egui_autocomplete::AutoCompleteTextEdit;
//! struct AutoCompleteExample {
//!   // User entered text
//!   text: String,
//!   // A list of strings to search for completions
//!   inputs: Vec<String>,
//! }
//!
//! impl AutoCompleteExample {
//!   fn update(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
//!     ui.add(AutoCompleteTextEdit::new(
//!        &mut self.text,
//!        &self.inputs,
//!        10,
//!     ));
//!   }
//! }
//! ````
use egui::{Context, Id, Key, Modifiers, TextBuffer, TextEdit, Widget};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::{min, Reverse};

impl<'a> AutoCompleteTextEdit<'a> {
    /// Creates a new [`AutoCompleteTextEdit`].
    ///
    /// `text_field` - Contents of the text edit passed into [`egui::TextEdit`]
    /// `search` - Slice of strings to use as the search term
    /// `max_suggestions` - Limit of max_suggestions to show in the drop down
    pub fn new(text_field: &'a mut String, search: &'a [String], max_suggestions: usize) -> Self {
        Self {
            text_field,
            search,
            max_suggestions,
        }
    }
}

/// An extension to the [`egui::TextEdit`] that allows for a dropdown box with autocomplete to popup while typing.  
pub struct AutoCompleteTextEdit<'a> {
    /// Contents of text edit passed into [`egui::TextEdit`]
    text_field: &'a mut dyn TextBuffer,
    /// Slice of strings to use as the search term
    search: &'a [String],
    /// A limit that can be placed on the maximum number of autocomplete suggestions shown
    max_suggestions: usize,
}

impl<'a> Widget for AutoCompleteTextEdit<'a> {
    /// The response returned is the response from the internal text_edit
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            text_field,
            search,
            max_suggestions,
        } = self;

        let id = ui.next_auto_id();
        ui.skip_ahead_auto_ids(1);
        let mut state = AutoCompleteTextEditState::load(ui.ctx(), id).unwrap_or_default();

        // only consume up/down presses if the text box is focused. This overwrites default behavior
        // to move to start/end of the string
        let up_pressed = state.focused
            && ui.input_mut(|input| input.consume_key(Modifiers::default(), Key::ArrowUp));
        let down_pressed = state.focused
            && ui.input_mut(|input| input.consume_key(Modifiers::default(), Key::ArrowDown));
        let text_response = TextEdit::singleline(text_field).ui(ui);
        state.focused = text_response.has_focus();

        let matcher = SkimMatcherV2::default().ignore_case();

        let mut match_results = search
            .iter()
            .filter_map(|s| {
                let score = matcher.fuzzy_match(s, text_field.as_str());
                score.map(|x| (s, x))
            })
            .collect::<Vec<_>>();
        match_results.sort_by_key(|k| Reverse(k.1));

        if text_response.changed()
            || (state.selected_index.is_some()
                && state.selected_index.unwrap() >= match_results.len())
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
        if let (Some(index), true) = (
            state.selected_index,
            enter_pressed && ui.memory(|mem| mem.is_popup_open(id)),
        ) {
            text_field.replace(match_results[index].0)
        }
        egui::popup::popup_below_widget(ui, id, &text_response, |ui| {
            for (i, (output, _)) in match_results.iter().take(max_suggestions).enumerate() {
                let mut highlighed = if let Some(x) = state.selected_index {
                    x == i
                } else {
                    false
                };
                if ui.toggle_value(&mut highlighed, *output).clicked() {
                    text_field.replace(output);
                }
            }
        });
        if !text_field.as_str().is_empty() && text_response.has_focus() && !match_results.is_empty()
        {
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

/// Stores the currently selected index in egui state
#[derive(Clone, Default)]
struct AutoCompleteTextEditState {
    /// Currently selected index, is `None` if nothing is selected
    selected_index: Option<usize>,
    /// Whether or not the text edit was focused last frame
    focused: bool,
}

impl AutoCompleteTextEditState {
    /// Store the state with egui
    fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|d| d.insert_persisted(id, self));
    }

    /// Get the state from egui if it exists
    fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(id))
    }

    /// Updates in selected index, checks to make sure nothing goes out of bounds
    fn update_index(
        &mut self,
        down_pressed: bool,
        up_pressed: bool,
        match_results_count: usize,
        max_suggestions: usize,
    ) {
        self.selected_index = match self.selected_index {
            // Increment selected index when down is pressed, limit it to the number of matches and max_suggestions
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment_index() {
        let mut state = AutoCompleteTextEditState::default();
        assert_eq!(None, state.selected_index);
        state.update_index(false, false, 10, 10);
        assert_eq!(None, state.selected_index);
        state.update_index(true, false, 10, 10);
        assert_eq!(Some(0), state.selected_index);
        state.update_index(true, false, 2, 3);
        assert_eq!(Some(1), state.selected_index);
        state.update_index(true, false, 2, 3);
        assert_eq!(Some(1), state.selected_index);
        state.update_index(true, false, 10, 3);
        assert_eq!(Some(2), state.selected_index);
        state.update_index(true, false, 10, 3);
        assert_eq!(Some(2), state.selected_index);
    }
    #[test]
    fn decrement_index() {
        let mut state = AutoCompleteTextEditState::default();
        state.selected_index = Some(1);
        state.update_index(false, false, 10, 10);
        assert_eq!(Some(1), state.selected_index);
        state.update_index(false, true, 10, 10);
        assert_eq!(Some(0), state.selected_index);
        state.update_index(false, true, 10, 10);
        assert_eq!(None, state.selected_index);
    }
}
