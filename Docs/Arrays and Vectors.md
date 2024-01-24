# Arrays and Vectors

## Arrays

Arrays are data structures of a fixed size, they can be one, two, or multi(n)-dimensional (see ndarray section below)

Arrays are great when manipulating data of fixed dimensions.

They're stored in contiguous memory, and take the form `[T; N]` where `T` is the type of the elements and `N` is the number of elements in the array aka the size or length of the array.

## Vectors

Vectors are contiguous, growable array types - effectively extensible, 1 dimensional arrays. 
In Rust, their size and thus memory requirements would be computed at compile time in order to efficiently
allocate memory for them.

## Ndarray

The `ndarray` crate provides support for n-dimensional arrays, which have particular utility in machine learning.
