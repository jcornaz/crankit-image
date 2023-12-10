# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [0.1.3] - 2023-12-10

### Added

* Ability to split an image into columns or rows.


## [0.1.2] - 2023-12-05


### Documentation

* Better document required feature flags on docs.rs


## [0.1.1] - 2023-11-26

* `anyhow` feature flag which provide conversions from error types to `anyhow::Error`.


## [0.1.0] - 2023-11-23

* `LoadImage` trait
* `DrawImage` trait
* `HasSize` trait
* `DrawFromOrigin` trait
* `DrawMode` enum
* Implementation of those traits for the types in [`playdate-sys`](https://docs.rs/playdate-sys/0.2) behind the feature
flag `playdate-sys-v02` (enabled by default).


[Unreleased]: https://github.com/jcornaz/crankit-image/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/jcornaz/crankit-image/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jcornaz/crankit-image/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jcornaz/crankit-image/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/crankit-image/compare/...v0.1.0
