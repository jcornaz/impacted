# impacted

2d collision test for arbitrary convex shapes in rust


## Usage example

```rust
use impact::prelude::*;

// Create a circle
let circle = shapes::Circle::new(1.0);
let circle_transform = Transform::default();

// Create a rectangle
let rect = shapes::Rectangle::from_half_extents(Vec2::splat(2.0));
let rect_transform: Transform = Affine2::from_translation(Vec2::new(2.0, 0.0)).try_into()?;

// Test if the shapes collide
assert!(impacted::are_collided(
    &TransformedShape::new(&circle_transform, &circle),
    &TransformedShape::new(&rect_transform, &rect)
));
```

Note: This example is using the provided `Circle` and `Rectangle` shapes to be as simple as possible.
But one can create their own shapes by implementing a `Support` trait (as this crate is using a [GJK] algorithm)

[GJK]: https://en.wikipedia.org/wiki/Gilbert%E2%80%93Johnson%E2%80%93Keerthi_distance_algorithm


## Cargo features

* `std` (enabled by default) Allow to use the standard library (need to be disabled for `no_std` crates)
* `bevy-transform-06` Conversion for bevy (version 0.6) `GlobalTransform`
* `glam-020` Conversion for glam (version 0.20) `Affine2`


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
