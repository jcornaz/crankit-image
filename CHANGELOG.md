# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


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


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/beancount_parser_2/compare/...v0.1.0
