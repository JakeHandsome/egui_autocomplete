# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2023-08-16 

### Breaking
- API has moved to a builder pattern, with only the required parameters in the `new` function
    - Max suggestions is no longer in the `new` function. To get the same behavior change usage as follows  
        **Old**
        ```rust
        AutoCompleteTextEdit::new(&mut text, &inputs, max_suggestions);
        ```
        **New**
        ```rust
        AutoCompleteTextEdit::new(&mut text, &inputs).max_suggestions(max_suggestions);
        ```

### Added
- API to change max_suggestions `.max_suggestions(usize)`
- API to enable highlighting of the matched characters `.highlight_matches(bool)`
- API to modify the internal egui TextEdit `.set_text_edit_properties`
    - See docs for example for this

[0.2.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/0.1.2...0.2.0

## [0.1.2] - 2023-08-07

### Added
- `Tab` can now be used to accept currently highlighted autocomplete 

## [0.1.1] 

### Fixed
- Added `serde` feature to support when running egui with `persistence` feature

## [0.1.0] 
- Initial Release

[0.1.2]: https://github.com/JakeHandsome/egui_autocomplete/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/JakeHandsome/egui_autocomplete/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/JakeHandsome/egui_autocomplete/releases/tag/0.1.0
