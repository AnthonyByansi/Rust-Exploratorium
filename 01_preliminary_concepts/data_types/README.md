# Rust Data Types

Rust provides a rich set of data types to help you write safe and efficient code. These data types cover a wide range of needs, from basic numeric types to custom user-defined types. Here's an overview of the key data types in Rust:

## 1. Integer Types

Rust offers signed and unsigned integer types with different bit sizes:
- `i8`, `i16`, `i32`, `i64`, `i128`: Signed integers
- `u8`, `u16`, `u32`, `u64`, `u128`: Unsigned integers
- `isize` and `usize`: Platform-dependent signed and unsigned integers

## 2. Floating-Point Types

Rust supports two floating-point types:
- `f32`: 32-bit floating-point number (single precision)
- `f64`: 64-bit floating-point number (double precision)

## 3. Boolean Type

The `bool` type represents either `true` or `false`, used for logical operations.

## 4. Character Type

The `char` type represents a single Unicode character.

## 5. String Types

Rust provides two string types:
- `&str`: A string slice, a reference to a string, often used for read-only access.
- `String`: A growable, heap-allocated string.

## 6. Array Types

Arrays in Rust have a fixed size and consist of elements of the same type: `[T, N]`, where `T` is the element type, and `N` is the number of elements.

## 7. Slice Types

Slices provide a dynamically-sized view into a contiguous sequence of elements of type `T`: `&[T]`.

## 8. Tuple Types

Tuples are ordered collections of elements, each with its own type: `(T1, T2, ...)`. 

## 9. Function Types

The `fn` type represents a function type.

## 10. Pointer Types

Rust has raw pointers to data types `T` denoted as `*const T` (immutable) and `*mut T` (mutable), useful for low-level memory manipulation.

## 11. Reference Types

References are used to borrow data:
- `&T`: Immutable reference to data of type `T`.
- `&mut T`: Mutable reference to data of type `T`.

## 12. Option Types

`Option<T>` represents an optional value that can be either `Some(T)` with a value of type `T` or `None`.

## 13. Result Type

`Result<T, E>` represents the result of an operation, either `Ok(T)` with a value of type `T` or `Err(E)` with an error of type `E`.

## 14. Enumeration Types (Enums)

Enums (`enum`) define custom data types representing one of a finite set of values.

## 15. Struct Types

Structs (`struct`) allow you to create custom data types by grouping multiple values with different types.

## 16. Union Types

Unions (`union`) enable a type to hold one of its fields at a time, primarily used for low-level memory manipulation.

## 17. Closures

Closures come in three types: `fn`, `fnMut`, and `fnOnce`, which are function-like types used to define closures.

## 18. Custom Types

You can define custom data types using `struct` and `enum` declarations, allowing you to create complex data structures.

## 19. Slices and String Slices

Slices and string slices (`&str`) are references to parts of arrays and strings, respectively, providing a view into a subset of the data.

## 20. References to Traits

References to traits (e.g., `&dyn Trait`) enable trait objects for dynamic dispatch, allowing polymorphism in Rust.
