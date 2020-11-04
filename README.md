# packed-integers

`packed_integers` provides a growable array for integer types in the range `u1` to `u31`.

## Use case

Assume you have a sequence of unsigned integers in the range [0, 100000] that you would like to
hold in memory. That range of values can be represented using 17 bits per integer, since
2<sup>17</sup> - 1 = 131071. As Rust has no `u17` type, you would typically store these values
in a `u32` array, wasting 15 bits per integer.

`packed-integers` helps alleviate this issue by packing these integers at the bit level,
essentially trading time for space.

## API

Where possible, `packed-integers` mimics the API for Rust's `Vec` in order to provide a set of
methods you're probably already familiar with.

## Inspiration / Resources

* [PackedArray][1]

[1]: https://github.com/gpakosz/PackedArray
