# Changelog

This project adheres to [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html).


## [unreleased]

### Added

- `IntoResult` implementation for `Mutex`.

### Changed

- Split the crate into modules. The `IntoResult` trait
  used for the `bind` macro is now in the `bind` module.
  So now you have to implement `el_macro::bind::IntoResult`
  for your own types instead of just `el_macro::IntoResult`.


## [0.2.2] - 2025-08-29

### Fixed

- Restored the ability to create mutable variable bindings
  with error handlers inlined using the `bind` macro.


## [0.2.1] - 2025-08-25

### Added

- README.md file.


## [0.2.0] - 2025-08-25

### Added

- CHANGELOG.md file.
- LICENSE file.
- Documentation comments for the `IntoResult` trait.
- Syntax description in the documentation comments for the `bind` macro.
- Optional match guard argument for the `if_matches` macro.
- Documentation comments for the `if_matches` macro.

### Changed

- `if_matches` macro now takes a mapping closure body instead of the entire closure. 


## [0.1.1] - 2025-08-22

Documented the `bind` macro.

### Added

- Documentation comments with examples for the `bind` macro.


## [0.1.0] - 2025-08-21

Initial version.

### Added

- `bind` macro.
- `IntoResult` trait.
- `IntoResult` implementations for `Result` and `Option`.
- `if_matches` macro.


[0.2.2]: https://github.com/dmitry-glvch/el-macro/tree/v0.2.2
[0.2.1]: https://github.com/dmitry-glvch/el-macro/tree/v0.2.1
[0.2.0]: https://github.com/dmitry-glvch/el-macro/tree/v0.2.0
[0.1.1]: https://github.com/dmitry-glvch/el-macro/tree/v0.1.1
[0.1.0]: https://github.com/dmitry-glvch/el-macro/tree/v0.1.0
