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
//!     ));
//!   }
//! }
//! ````
use egui::{text::LayoutJob, Context, FontId, Id, Key, Modifiers, TextBuffer, TextEdit, Widget};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::{min, Reverse};

/// Trait that can be used to modify the TextEdit
type SetTextEditProperties = dyn FnOnce(TextEdit) -> TextEdit;

/// An extension to the [`egui::TextEdit`] that allows for a dropdown box with autocomplete to popup while typing.
pub struct AutoCompleteTextEdit<'a, T> {
    /// Contents of text edit passed into [`egui::TextEdit`]
    text_field: &'a mut String,
    /// Data to use as the search term
    search: T,
    /// A limit that can be placed on the maximum number of autocomplete suggestions shown
    max_suggestions: usize,
    /// If true, highlights the macthing indices in the dropdown
    highlight: bool,
    /// Used to set properties on the internal TextEdit
    set_properties: Option<Box<SetTextEditProperties>>,
}

impl<'a, T, S> AutoCompleteTextEdit<'a, T>
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    /// Creates a new [`AutoCompleteTextEdit`].
    ///
    /// `text_field` - Contents of the text edit passed into [`egui::TextEdit`]
    /// `search` - Data use as the search term
    pub fn new(text_field: &'a mut String, search: T) -> Self {
        Self {
            text_field,
            search,
            max_suggestions: 10,
            highlight: false,
            set_properties: None,
        }
    }
}

impl<'a, T, S> AutoCompleteTextEdit<'a, T>
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    /// This determines the number of options appear in the dropdown menu
    pub fn max_suggestions(mut self, max_suggestions: usize) -> Self {
        self.max_suggestions = max_suggestions;
        self
    }
    /// If set to true, characters will be highlighted in the dropdown to show the match
    pub fn highlight_matches(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Can be used to set the properties of the internal [`egui::TextEdit`]
    /// # Example
    /// ```rust
    /// # use egui_autocomplete::AutoCompleteTextEdit;
    /// # fn make_text_edit(mut search_field: String, inputs: Vec<String>) {
    /// AutoCompleteTextEdit::new(&mut search_field, &inputs)
    ///     .set_text_edit_properties(|text_edit: egui::TextEdit<'_>| {
    ///         text_edit
    ///             .hint_text("Hint Text")
    ///             .text_color(egui::Color32::RED)
    ///     });
    /// # }
    /// ```
    pub fn set_text_edit_properties(
        mut self,
        set_properties: impl FnOnce(TextEdit) -> TextEdit + 'static,
    ) -> Self {
        self.set_properties = Some(Box::new(set_properties));
        self
    }
}

impl<'a, T, S> Widget for AutoCompleteTextEdit<'a, T>
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    /// The response returned is the response from the internal text_edit
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            text_field,
            search,
            max_suggestions,
            highlight,
            set_properties,
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

        let mut text_edit = TextEdit::singleline(text_field);
        if let Some(set_properties) = set_properties {
            text_edit = set_properties(text_edit);
        }

        let text_response = text_edit.ui(ui);
        state.focused = text_response.has_focus();

        let matcher = SkimMatcherV2::default().ignore_case();

        let mut match_results = search
            .into_iter()
            .filter_map(|s| {
                let score = matcher.fuzzy_indices(s.as_ref(), text_field.as_str());
                score.map(|(score, indices)| (s, score, indices))
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

        let accepted_by_keyboard = ui.input_mut(|input| input.key_pressed(Key::Enter))
            || ui.input_mut(|input| input.key_pressed(Key::Tab));
        if let (Some(index), true) = (
            state.selected_index,
            ui.memory(|mem| mem.is_popup_open(id)) && accepted_by_keyboard,
        ) {
            text_field.replace_with(match_results[index].0.as_ref())
        }
        egui::popup::popup_below_widget(ui, id, &text_response, |ui| {
            for (i, (output, _, match_indices)) in
                match_results.iter().take(max_suggestions).enumerate()
            {
                let mut selected = if let Some(x) = state.selected_index {
                    x == i
                } else {
                    false
                };

                let text = if highlight {
                    highlight_matches(
                        output.as_ref(),
                        match_indices,
                        ui.style().visuals.widgets.active.text_color(),
                    )
                } else {
                    let mut job = LayoutJob::default();
                    job.append(output.as_ref(), 0.0, egui::TextFormat::default());
                    job
                };
                if ui.toggle_value(&mut selected, text).clicked() {
                    text_field.replace_with(output.as_ref());
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

/// Highlights all the match indices in the provided text
fn highlight_matches(text: &str, match_indices: &[usize], color: egui::Color32) -> LayoutJob {
    let mut formatted = LayoutJob::default();
    let mut it = text.char_indices().peekable();
    let mut char_idx = 0; // Track char index
    // Iterate through all indices in the string
    while let Some((j, _)) = it.next() {
        let start = j;
        let mut end = j;
        let match_state = match_indices.contains(&char_idx);
        // Find all consecutive characters that have the same state
        while let Some((_, c)) = it.peek() {
            if match_state == match_indices.contains(&(char_idx + 1)) {
                end += c.len_utf8();
                // Advance the iterator, we already peeked the value so it is fine to ignore
                _ = it.next();
            } else {
                break;
            }
        }
        // Format current slice based on the state
        let format = if match_state {
            egui::TextFormat::simple(FontId::default(), color)
        } else {
            egui::TextFormat::default()
        };
        let slice = &text[start..=end];
        formatted.append(slice, 0.0, format);
        char_idx += 1;
    }
    formatted
}

/// Stores the currently selected index in egui state
#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
        let mut state = AutoCompleteTextEditState {
            selected_index: Some(1),
            ..Default::default()
        };
        state.selected_index = Some(1);
        state.update_index(false, false, 10, 10);
        assert_eq!(Some(1), state.selected_index);
        state.update_index(false, true, 10, 10);
        assert_eq!(Some(0), state.selected_index);
        state.update_index(false, true, 10, 10);
        assert_eq!(None, state.selected_index);
    }
    #[test]
    fn highlight() {
        let text = String::from("Test123");
        let match_indices = vec![1, 5, 6];
        let layout = highlight_matches(&text, &match_indices, egui::Color32::RED);
        assert_eq!(4, layout.sections.len());
        let sec1 = layout.sections.get(0).unwrap();
        assert_eq!(sec1.byte_range, 0..1);
        assert_ne!(sec1.format.color, egui::Color32::RED);

        let sec2 = layout.sections.get(1).unwrap();
        assert_eq!(sec2.byte_range, 1..2);
        assert_eq!(sec2.format.color, egui::Color32::RED);

        let sec3 = layout.sections.get(2).unwrap();
        assert_eq!(sec3.byte_range, 2..5);
        assert_ne!(sec3.format.color, egui::Color32::RED);

        let sec4 = layout.sections.get(3).unwrap();
        assert_eq!(sec4.byte_range, 5..7);
        assert_eq!(sec4.format.color, egui::Color32::RED);
    }
}
