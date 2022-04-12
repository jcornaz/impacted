# impacted

[![License](https://img.shields.io/github/license/jcornaz/impacted)](https://github.com/jcornaz/impacted/blob/main/UNLICENSE)
[![Crates.io](https://img.shields.io/crates/v/impacted)](https://crates.io/crates/impacted)
[![Docs](https://img.shields.io/docsrs/impacted)](https://docs.rs/impacted)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/impacted/build)](https://github.com/jcornaz/impacted/actions)


2d collision test for game-development in rust (with optional integration and example for bevy)

This provides a low-level "narrow-phase" collision-detection logic,
with some integration for the [bevy] game engine (behind feature flag).

If you want to pair it with a broad-phase, you may look at [bvh-arena] or [broccoli].

[bevy]: https://bevyengine.org
[bvh-arena]: https://github.com/jcornaz/bvh-arena
[broccoli]: https://github.com/tiby312/broccoli


## Roadmap

* [x] collision test (GJK implementation)
* [ ] collision shapes
  * [x] circle
  * [x] rectangle
  * [ ] convex-hull
  * [ ] capsule
* [x] contact generation (EPA implementation)


## Usage example

```rust
use impacted::{CollisionShape, Transform};
use glam::Vec2; // <-- use any math library you like

// Create a circle
let circle = CollisionShape::new_circle(1.0);

// Create a rectangle
let mut rect = CollisionShape::new_rectangle(4.0, 4.0)
    .with_transform(Transform::from_translation(Vec2::new(2.0, 0.0)));

// Test for collision
assert!(circle.is_collided_with(&rect));
```

You may also look in the [examples](https://github.com/jcornaz/impacted/tree/main/examples) directory
for more complete/concrete usage examples (e.g. using the bevy engine)


## Installation

<!--- x-release-please-start-version --->
```toml
[dependencies]
impacted = "1.0.0"
```
<!--- x-release-please-end-version --->


### For bevy users
<!--- x-release-please-start-version --->
```toml
[dependencies]
impacted = { version = "1.0.0", features = ["bevy-06"] }
```
<!--- x-release-please-end-version --->


## Cargo features

* `std` (enabled by default) Allow to use the standard library (need to be disabled for `no_std` crates)
* `bevy-06` All [bevy](https://bevyengine.org) 0.6 interop (alias for `["bevy-transform-06", "bevy-ecs-06"]`)
* `bevy-transform-06` Interoperability with [bevy_transform](https://crates.io/crates/bevy_transform) 0.6
* `bevy-ecs-06` Interoperability with [bevy_ecs](https://crates.io/crates/bevy_ecs) 0.6
* `bvh-arena` Interoperability with [bvh-arena](https://crates.io/crates/bvh-arena) bounding volumes


## MSRV

The minimum supported rust version is currently: `1.58`

It may be increased to a newer stable version in a minor release.

It will be increased to the latest stable version in a major release.


## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
