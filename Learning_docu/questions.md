# My questions on Rust

1. Does `println!` own data on heap? is it act as a `function`

```rust

let y: Option<String>  = Some(String::from("this is my -> y <- string ... allocated on the heap as well" ));

match &y {
    Some(s) => println!("Yes it's exited and it is => {:#?}", s),
    None => println!("No it's exited and it is => None",)
}

println!("what is the value of right now ... ? {:#?}", y);
println!("what is the value of right now ... ? {:#?}", y);
println!("what is the value of right now ... ? {:#?}", y);


```
we we can print as many of y without references?

2. Access element from vec! and also push in same time, `rust book` `listing
   8-6`: Attempting to add an element to a vector while holding a reference to
   an item.

```rust
    let mut vec = vec![1, 2, 3, 4, 5];
    let first_element = &vec[0];

    vec.push(6);
    println!("The first element is : {:#?}", &vec[0]);

```


3. If you don’t know the exhaustive set of types a program will get at runtime
   to store in a vector, the enum technique won’t work. Instead, you can use a
   trait object, which we’ll cover in Chapter 17. ?  Chapter 8-9: enum with vec
   different types.



