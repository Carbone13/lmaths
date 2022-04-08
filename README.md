# L-Maths

[![Documentation](https://docs.rs/lmaths/badge.svg)](https://docs.rs/lmaths/latest/lmaths/)
[![Version](https://img.shields.io/crates/v/lmaths.svg)](https://crates.io/crates/lmaths)
[![License](https://img.shields.io/crates/l/lmaths.svg)](https://github.com/carbone13/lmaths/blob/master/LICENSE)

Short 2D Maths library. Currently implements Vector2 (64-bit floats) and Point2 (integer), with their classic functions such as `length()`, `normalize()`, `dot()` etc...

Nothing fancy, just a good base for my projects.

Planning to implement other useful structs, like Matrices.

Examples :

Creating a Vector2 :
```rust
let v1 = Vector2::new(1.0, 5.6); //
// or using constants
let v2 = Vector2::ZERO; // (0.0, 0.0)
```
Same process for Point2 :
```rust
let p1 = Point2::new(-56, 45); //
// or using constants
let p2 = Point2::X_UNIT; // (1, 0)
```
Some demo for the functions :
```rust
let v3 = v1.normalized(); // won't modify v1
v1.normalize() // will modify v1
let dp = v1.dot(v2);
// or
let dp = Vector2::dot(v1, v2);
```