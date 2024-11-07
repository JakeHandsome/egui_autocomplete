# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [7.1.0] - 2024-11-07

### Added
This sets the changed flag in the Response object if the text was changed by selection from list. This can be used like this: [#28](https://github.com/JakeHandsome/egui_autocomplete/pull/28)
    ```rust
        if ui
            .add(AutoCompleteTextEdit::new(text, autocomplete_list))
            .changed()
        {
            //do something
        };
    ```

### Fixed
- Removed `eframe` as dependency, it is not required for this library  [#27](https://github.com/JakeHandsome/egui_autocomplete/pull/27)

## [7.0.0] - 2024-07-21
### Breaking
- Updating to egui 0.28
    - This update had some changes to popup logic, so there may be some difference when clicking to accept a value

[7.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/6.0.1...7.0.0

## [6.0.1] - 2024-06-07

### Fixed
- Fixed crash that occured when unicode characters where searched. Thanks [@zaaarf](https://github.com/zaaarf) [#24](https://github.com/JakeHandsome/egui_autocomplete/pull/24)

[6.0.1]: https://github.com/JakeHandsome/egui_autocomplete/compare/6.0.0...6.0.1

## [6.0.0] - 2024-03-30

### Breaking
- Update to egui 0.27

[6.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/5.0.0...6.0.0

## [5.0.0] - 2024-02-11

### Breaking
- Update to egui 0.26

[5.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/4.0.0...5.0.0

## [4.0.0] - 2024-01-18

### Breaking
- Update to egui 0.25

[4.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/3.0.0...4.0.0

## [3.0.0] - 2023-10-16

### Breaking
- Update to egui 0.24. 
- MSRV bumped to 1.72.0

[3.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/2.0.0...3.0.0

## [2.0.0] - 2023-10-16

### Breaking
- `AutoCompleteTextEdit::new` function now takes a `impl IntoIterator<Item=impl AsRef<str>>` instead of `&[String]` [#16](https://github.com/JakeHandsome/egui_autocomplete/pull/16) thanks [@JiangengDong](https://github.com/JiangengDong)

[2.0.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/1.0.0...2.0.0

## [1.0.0] - 2023-09-30 

### Breaking
- `egui` dependency updated to 0.23
- MSRV bumped to 1.70.0

[0.2.0]: https://github.com/JakeHandsome/egui_autocomplete/compare/0.2.0...1.0.0

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
