# Helper Idomtics
This is a collection of many useful `API` functions and other definitions which are used almost on reach rust project.



1. meaning of `Option<T>`
```rust
let x : Option<T> {
        None,
        Some(T)
    }


```
2. Meaning of `match`
```rust
// using match
match x {
        None => println!("the value is not existed ..."),
        Some(val) => println!("we got the value {}", val)
    }
```

3. Using `Option` enum with `String` checking `match`,
- **NOTE** we must specify the `&y` at the match as we will use the value again
in the `println!` and just borrow it not own it.

```rust
let y: Option<String>  = Some(String::from("this is my -> y <- string ... allocated on the heap as well" ));

match &y {
    Some(s) => println!("Yes it's exited and it is => {:#?}", s),
    None => println!("No it's exited and it is => None",)
}

println!("what is the value of right now ... ? {:#?}", y);

```

4. borrowed form means (&), for example **&str** which is a string slice, that
   we call it `string literal`.

5. Possibility of converating `String` object to `&str` literal
This was a challenging as we need to understand more about the `lifetime`
- This can speed up significantly our work on `string` as we are dealing with `&str` not `String`.

```rust

    let mut v : Vec<&str> = Vec::new();
    for i in 0..10{
        let x = format!("we have value -> {i}");
        let s: &'static str = super::common_collections::string_to_static_str(x);
        //let y:&str = &x[..]; <- this doesnt work
        v.push(s);

    }

    for item in &v{
        // you are passing here item which is a reference, (memory address), you need to dererence it at first.
        println!("We have now -> {}, with type -> {}", item, super::common_collections::type_of(*item));
    }



}

// This function will take String object and give us &str literal, It is implmentation of Rust 1.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
// this function will tell us the type of object in Rust
fn type_of<T>(_:T) -> & 'static str {
        std::any::type_name::<T>()
    }
```
- [Reference 01 ](https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str)
- [Reference 02 ](https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8/41034751#41034751)
- [Reference 03 ](https://users.rust-lang.org/t/how-check-type-of-variable/33845)

6.
