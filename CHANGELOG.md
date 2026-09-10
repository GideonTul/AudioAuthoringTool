# Changelog

## 9/9/2026

### Added
* Voice and VoiceMixer structs

### Changed
* Simple player now uses the new structs rather than AudioRendeder

### Breaking Change

### Deprecated

### Removed

### Fixed

## 9/4/2026

### Added

### Changed

### Breaking Change
* Removed basic-rust-example

### Deprecated

### Removed
* Removed basic-rust-example

### Fixed

## 8/10/2026

### Added
* The errors should be more graceful

### Changed
* Refactored AudioBackend to a new file
* Changed AudioBackend new() callback to directly use AudioFormat

### Breaking Change
* The Callback change

### Deprecated
* basic-rust-example will be removed in the future.

### Removed

### Fixed

## 8/4/2026

### Added
* Added Symphonia wrapper for reading audio
* Added simple-player.rs example

### Changed
* Removed all logic from audio-player/src/main.rs
* Moved basic rust examples to its own folder

### Breaking Change
* the code in basic-rust-example may not work since it has been moved around.

### Deprecated
* basic-rust-example will be removed in the future.

### Removed

### Fixed

## 7/29/2026

### Added
* Added directory structure instructions to README
* Added full Symphonia implementation

### Changed
* Changed directory structure

### Deprecated

### Removed

### Fixed

## 7/24/2026

### Added
* Added a closure based render for the callback in cpal

### Changed
* Refactored audio-player/src/main.rs into lib/audio-backend/src/lib.rs

### Deprecated

### Removed

### Fixed