# Useful Rust API

## How to know the type

```rust
fn print_type_of<T>(_: &T) -> String {
    let mut my_string_output = String::new();
    my_string_output.push_str(std::any::type_name::<T>());
    //format!("{}", std::any::type_name::<T>())
    my_string_output
}
```

## Getting the type exclusive

```rust
println!("{}", std::any::type_name::<i32>());
println!("{}", std::any::type_name::<i64>());
println!("{}", std::any::type_name::<i128>());
println!("{}", std::any::type_name::<f32>());
println!("{}", std::any::type_name::<f64>());
println!("{}", std::any::type_name::<bool>());
println!("{}", std::any::type_name::<[i32]>());
println!("{}", std::any::type_name::<[&i32]>());
println!("{}", std::any::type_name::<Vec<i32>>());
println!("{}", std::any::type_name::<String>());
```

## How to get byte representation of requested variable

```rust

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmtr.write_fmt(format_args!("{:02x}", byte));
        }
        Ok(())
    }
}

let mut buff = [0_u8; 2];
buff[0] = 123;

println!("{:x}", ByteBuf(&buff));
```


