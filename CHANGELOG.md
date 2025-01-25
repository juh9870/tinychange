# Changelog

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Fixed

- Destroying of description of unreleased sections without categories in them (by juh9870)

## [0.3.1] - 2025-01-24

### Changed

- Updated README.md (by juh9870)

## [0.3.0] - 2025-01-24

### Added

- Cargo dist for building binaries and NPM release (by juh9870)
- Checks to ensure that CLI doesn't override unexpected parts of the 0.3.0 section (by juh9870)
- CLI version number (by juh9870)
- Better CLI help messages (by juh9870)
- License files (by juh9870)
- Initial sanity check tests (by juh9870)

### Changed

- Avoid adding unnecessary whitespace when inserting unreleased section (by juh9870)
- Fetch git username by running `git` commands, instead of pulling gix package (by juh9870)

## [0.2.0] - 2025-01-24

### Added

- Readme file (by juh9870)

### Removed

- Public visibility of internal modules (by juh9870)

## [0.1.0] - 2025-01-24

### Added

- Merging functionality (by juh9870)
- `--keep` flag for `merge` command (by juh9870)

### Changed

- Abort merge if no tinychanges are found (by juh9870)
- Better message prompt message (by juh9870)

<!-- next-url -->
[Unreleased]: https://github.com/juh9870/tinychange/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/juh9870/tinychange/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/juh9870/tinychange/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/juh9870/tinychange/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/juh9870/tinychange/tree/v0.1.0