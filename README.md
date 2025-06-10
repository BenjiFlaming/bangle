# Bangle

The `bangle` crate provides strongly-typed floating point angles,
specified in radians, degrees, rotations, or percentages,
with efficient and ergonomic conversion between units.
Both `f32` and `f64` numeric types are supported,
with `f32` being the implicit default.
All conversions, for each unit and numeric type,
are covered by tests, and the library is fully `no_std` compatible.
`bangle` is especially intended for situations where local code clarity
may be enhanced by specifying or modifying an angle in units other than radians.

