# impacted

[![License](https://img.shields.io/crates/l/impacted)](#Unlicense)
[![Crates.io](https://img.shields.io/crates/v/impacted)](https://crates.io/crates/impacted)
![rustc](https://img.shields.io/badge/rustc-1.60+-blue?logo=rust)
[![Docs](https://docs.rs/impacted/badge.svg)](https://docs.rs/impacted)

2d collision test for game-development in rust

This provides a low-level "narrow-phase" collision-detection logic.

If you want to pair it with a broad-phase, you may look at [bvh-arena] or [broccoli].

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
for more complete/concrete usage examples.


## Installation

```sh
cargo add impacted
```

## Cargo features

| Feature             | Status             | Description                                                                       |
|---------------------|--------------------|-----------------------------------------------------------------------------------|
| `std`               | Enabled by default | Allow to use rust the standard library (need to be disabled for `no_std` crates)  |
| `bvh-arena`         |                    | Integration with [bvh-arena](https://crates.io/crates/bvh-arena) bounding volumes |
| `bevy-ecs-08`       | ***Deprecated***   | Integration with [bevy_ecs](https://crates.io/crates/bevy_ecs) 0.8                |
| `bevy-transform-08` | ***Deprecated***   | Integration with [bevy_transform](https://crates.io/crates/bevy_transform) 0.8    |
| `bevy-08`           | ***Deprecated***   | All bevy 0.8 integrations (alias for `["bevy-ecs-08", "bevy-transform-08"]`)      |
| `bevy-ecs-07`       | ***Deprecated***   | Integration with [bevy_ecs](https://crates.io/crates/bevy_ecs) 0.7                |
| `bevy-transform-07` | ***Deprecated***   | Integration with [bevy_transform](https://crates.io/crates/bevy_transform) 0.7    |
| `bevy-07`           | ***Deprecated***   | All bevy 0.7 integrations (alias for `["bevy-ecs-07", "bevy-transform-07"]`)      |
| `bevy-ecs-06`       | ***Deprecated***   | Integration with [bevy_ecs](https://crates.io/crates/bevy_ecs) 0.6                |
| `bevy-transform-06` | ***Deprecated***   | Integration with [bevy_transform](https://crates.io/crates/bevy_transform) 0.6    |
| `bevy-06`           | ***Deprecated***   | All bevy 0.6 integrations (alias for `["bevy-ecs-06", "bevy-transform-06"]`)      |

> **Warning**
>
> Feature flags not listed in the table above are not part of the public API and are subject to breaking changes

> **Note**
>
> There won't be any integration for the upcoming bevy versions.
>
> Though, `impacted` can still be used from a bevy project. Look at the [examples](https://github.com/jcornaz/impacted/tree/main/examples) for more details. 

## MSRV

The minimum supported rust version is currently: `1.60`

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
