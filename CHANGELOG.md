# Changelog

All notable changes are documented in this file.

Unreleased changes (if any) can be found in the latest [release pull-request].

[release pull-request]: https://github.com/jcornaz/impacted/pulls?q=is%3Apr+is%3Aopen+label%3A%22autorelease%3A+pending%22


## [1.3.0](https://github.com/jcornaz/impacted/compare/v1.2.0...v1.3.0) (2022-02-20)


### Features

* add interop with the crate `bvh-arena` ([#13](https://github.com/jcornaz/impacted/issues/13)) ([c53d308](https://github.com/jcornaz/impacted/commit/c53d308143a5da9b0478bed05010e2bc6aed54d1))
* segment collision shape ([#11](https://github.com/jcornaz/impacted/issues/11)) ([17c8de5](https://github.com/jcornaz/impacted/commit/17c8de5da5068d129ead93c47d3f28a18281738e))

## [1.2.0](https://github.com/jcornaz/impacted/compare/v1.1.0...v1.2.0) (2022-02-19)


### Features

* give access to `shape_data` information from `CollisionShape` ([#7](https://github.com/jcornaz/impacted/issues/7)) ([cc60f4e](https://github.com/jcornaz/impacted/commit/cc60f4eab816fd072d2fd1c60d8b7491d82f06c9))


## [1.1.0](https://github.com/jcornaz/impacted/compare/v1.0.0...v1.1.0) (2022-02-01)


### Features

* contact data generation ([#3](https://github.com/jcornaz/impacted/issues/3)) ([cbc9a0d](https://github.com/jcornaz/impacted/commit/cbc9a0dd9853425c77d70086b6411dbb8b055cf9))


### Bug Fixes

* consider collided shapes that have an interpenetration of exactly 0 ([cc2177a](https://github.com/jcornaz/impacted/commit/cc2177a50269920a07bd021e5a7769010242e449))


### Deprecation

* `Error` type that is not used ([0cb4b81](https://github.com/jcornaz/impacted/commit/0cb4b812bdc4b825c7ad99da13dc9b46929de7ed))



## 1.0.0 (2022-01-29)

### Features

* collision detection ([3891232](https://github.com/jcornaz/impacted/commit/389123278cf4c056c9e36e4d9985ddf1c05d5102))
* circle and rectangle shapes ([c369226](https://github.com/jcornaz/impacted/commit/c369226516bea6750b653ea544ffc151b5addfff))
* shape transform ([030dcde](https://github.com/jcornaz/impacted/commit/030dcde6807a42cb6b5fac4b14cad6e2e8c5455d))
* conversion for bevy transform ([d90b5f8](https://github.com/jcornaz/impacted/commit/d90b5f866d936c37809d54b7bdeb56d51cf0d098))
* support for bevy_ecs 0.6 ([0136d7d](https://github.com/jcornaz/impacted/commit/0136d7d4dc3f10ed1ed7b50e6b67c4884124168a))
