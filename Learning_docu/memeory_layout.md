# Memory layout

As we already know that there are two places in memory to stor the data, one is
on `stack` and the other one on `heap`, we can check both the `stack` and
`heap` in IDs such as `Microsoft Visual Studio IDE`. There is no memeory profiler in `Rust`,

## What is required

data structure that store data on `stack` are simple and the capacity is
alreayd known, compared to the `heap` allocation which has three pieces of
inormatin `ptr` which a pointer to the data on the `heap` , `length`
`capacity`. These three pices of information are called `buffer` on memeory.

// 0000-0000<->0000-0000<->0000-0000<->0000-0000 <- maximum number per byte is {255: decimal} or {ff: hex} or {1111-1111:binary}

### On Stack
- Assume we are examining the `i32` signed-interger.
```rust
let ptr: u32 = 255; // 0000-0000<->0000-0000<->0000-0000<->0000-0000 <- maximum number per byte is {255: decimal} or {ff: hex} or {1111-1111:binary}
// method only for float
//println!("{:#?}-> {:#?}", ptr.to_be_bytes(), ptr.to_bits());
println!(
    "value: {}, \nByte: {:b}\nMemory Address: {:p}\nHexadecimal: {:x}",
    ptr, ptr, &ptr, ptr
);
```

```shell
value: 255,
Byte: 11111111
Memory Address: 0x16d9ae0d4
Hexadecimal: ff

```

Although, we have specified `i32` which is `8-bytes` or `32-bits` when we print
it will show only the values that we specified in (`1`) not (`0`), which means
it print as (11111111) but it means (0000-0000x0000-0000x0000-0000x1111-1111)

- **Notice** that Maximum number that the byte can carry ove is
| idx | decimal number | representation type | value    |
|-----+----------------+---------------------+----------|
| 1   | 255            | binary              | 11111111 |
| 2   | -              | hex                 | ff       |

- but the complier will fill the bytes from left to right (the cheron)


### On Heap
