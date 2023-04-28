# Standard Library Data Structure.

## Stack allocated data structure
In Rust, the following data structures are allocated on the stack by default:

1. Primitive types: `bool`, `char`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`,
   `u16`, `u32`, `u64`, `u128`, `f32`, `f64`, and pointers such as `&T` and
   `*const T` or `*mut T`.
2. Arrays (`[T; N]`): Fixed-size arrays of elements of type T.
3. Tuples: Fixed-size groups of elements of different types.
4. Structs: User-defined data types that group together values of different
types.
- Note that although these data structures are allocated on the stack by
  default, they may contain references to data allocated on the heap.

## In Rust, the following data structures are allocated on the heap by default:

- `Box<T>`: A smart pointer that provides heap allocation of a value of type T.
- `Vec<T>`: A growable array that stores values of type T on the heap.
- `String`: A growable string that stores UTF-8 encoded data on the heap.
- `HashMap<K, V>`: A hash map that stores keys and values of arbitrary types on
  the heap.
- `HashSet<T>`: A set that stores values of type T on the heap.
- `Rc<T>`: A reference-counted pointer that provides shared ownership of a
  value of type T on the heap.
- `Arc<T>`: An atomic reference-counted pointer that provides shared ownership
  of a value of type T on the heap, with thread-safe reference counting.
- `Cell<T>`: A simple type that stores a single value of type T that can be
  mutated even when the cell is behind a shared reference. It is allocated on
  the stack, but can contain a value on the heap. Note that although these data
  structures are allocated on the heap by default, Rust also provides ways to
  allocate values on the stack or in other locations, depending on the needs of
  the program.


