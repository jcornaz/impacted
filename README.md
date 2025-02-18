# impacted

[![License](https://img.shields.io/crates/l/impacted)](#Unlicense)
[![Crates.io](https://img.shields.io/crates/v/impacted)](https://crates.io/crates/impacted)
![rustc](https://img.shields.io/badge/rustc-1.68+-blue?logo=rust)
[![Docs](https://docs.rs/impacted/badge.svg)](https://docs.rs/impacted)
![Maintenance](https://img.shields.io/maintenance/passively/2025)

2d collision test for game-development in rust

This provides a low-level "narrow-phase" collision-detection logic.

If you want to pair it with a broad phrase, you may look at [bvh-arena] or [broccoli].

[bvh-arena]: https://github.com/jcornaz/bvh-arena
[broccoli]: https://github.com/tiby312/broccoli

> [!NOTE]
>
> If you only need simple shapes (i.e. axis-aligned bounding boxes) you may prefer to look at this simpler 2d collision library I'm working on: [collision2d](https://github.com/jcornaz/collision2d)

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

* `std` (enabled by default) Allow to use rust the standard library (need to be disabled for `no_std` apps)
* `bvh-arena` Integration with [bvh-arena](https://crates.io/crates/bvh-arena) bounding volumes

## Unstable feature flags

**The following features may receive breaking changes or be removed in a patch release!**

* `unstable-v3`: enable the `v3` module containing an exploration of what could be the next major version of the API
* `unstable-v3-aabb`: Axis-Aligned-Bounding-Box shape for the `v3` module



## MSRV

The minimum supported rust version is currently: `1.68`

Bumping the minimum supported rust version to a newer stable version
is not considered a breaking change.


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
