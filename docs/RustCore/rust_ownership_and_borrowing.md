# Rust OwnerShip and Borrowing
The owenership and borrowing are why rust is rust.
- Initially rurstrated both new and senior developers.
- Memeory management.
    - No garbage colection
    - Developer manages memeory in code
    - stack vs heap
- benefit
    - Runtime speed
    - Parallel and concurrent processing
    - Safety


## Concept
Assume we have the following example

```rust
let stack_i8: i8 = 10;
let stack_f32: f32 = 20.;
let stack_bool: bool = true;
let stack_char: char = 'a';
```
They will be allocated in the `stack` as:
```
+---------------+
|   stack_char  |
+---------------+
+---------------+
|   stack_bool  |
+---------------+
+---------------+
|   stack_f32   |
+---------------+
+---------------+
|   stack_i8    |
+---------------+
```

### STACK
- Fast memeory creation  and retrieval .. speed , speed , speed!
- Memory is automatically recaptured by the program after variables go out of scope
- I sthe default in Rrust
- Fixed size variables .. Collections Cannot be stack based ( and String are a collection of u8 's')
####  Stack then,
- known, fixed size type in memeory
- No need to search, they stack on top of each others,
- very fast
- Once they cleaned they pop from top with the critera (`LIFO`), Last Inter First OUT.
