# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

 * Update parse-display dependency to 0.8
 * Bump MSRV to 1.65

## [0.3.0] - 2023-04-04

### Changed

 * Bump MSRV to 1.58
 * Make API endpoint futures safe to share between threads

## [0.2.0] - 2022-12-01

### Added

 * Support all parameters for List Movies with the `ListMovies` builder
 * Support for the movie details API endpoint

### Changed

 * Updated parse-display dependency to 0.6.0
 * Bump MSRV to 1.56

### Deprecated

 * `list_movies` is replaced by the `ListMovies` builder

## [0.1.1] - 2021-05-19

### Fixed

 * Fixed a panic when a search gave zero results

## [0.1.0] - 2021-05-19

Initial release

[Unreleased]: https://github.com/rnestler/yts-api-rs/compare/0.3.0...master
[0.3.0]: https://github.com/rnestler/yts-api-rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/rnestler/yts-api-rs/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/rnestler/yts-api-rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/rnestler/yts-api-rs/releases/tag/0.1.0
