# Bangle

Strongly-typed angles, specified in radians, degrees, rotations, or percentages,
with efficient and ergonomic conversion between formats.

This crate is especially intended to act as a bridge between functions or data structures
which need angles specified in radians, and programmers who (like me)
sometimes find it more intuitive to specify angles in other units.

Both 32-bit and 64-bit floating point numbers are supported,
with 32 bits being the default if no numeric type is explicitly specified.

