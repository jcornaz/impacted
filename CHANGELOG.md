# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [2.0.3] - 2024-09-02

### Dependencies

* Update glam to `0.29`


## [2.0.2] - 2024-06-12

### Breaking changes to `unstable-v3`

* Change `Shape` trait methods to return `impl Iterator`
* Rename feature `glam-0-24` to `unstable-glam-0-24`

### Dependencies

* Update glam to 0.28
* Bump rust MSRV to 1.68


## [2.0.1] - 2023-11-01

### Added

* `unstable-v3` feature flag and related `v3` module: an exploration of how could look like the next major version
* `unstable-v3-aabb` Axis-Aligned-Bounding-Box shape for the v3 module


## [2.0.0] - 2023-08-03

### Breaking changes

* Removed all bevy interoperability features.
  Note that this crate, despite being now engine agnostic, is still usable from bevy, and an example of usage from bevy is still provided.
* Removed deprecated and unused `Error` type


### Bug fixes

* Fix panic of contact generation that could occur if a circle was tested against its rotated version


### Dependencies

* Update glam to 0.23


### Documentation

* List feature flags in API documentation


## [1.5.3] - 2023-01-01


### Documentation

* lower minimum supported rust version (msrv) to 1.60 ([259b7a5](https://github.com/jcornaz/impacted/commit/259b7a57ee36a602d12eb86e083d2a2df6897649))
* **readme:** fix build badge ([1f91bf8](https://github.com/jcornaz/impacted/commit/1f91bf88ee4a57eddc4a1ed4b47fc5ffea04e85d))


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v2.0.3...HEAD
[2.0.3]: https://github.com/jcornaz/beancount_parser_2/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/jcornaz/beancount_parser_2/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/jcornaz/beancount_parser_2/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/jcornaz/beancount_parser_2/compare/v1.5.3...v2.0.0
[1.5.4]: https://github.com/jcornaz/impacted/compare/v1.5.3...v1.5.4
