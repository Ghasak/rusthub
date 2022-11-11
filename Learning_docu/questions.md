# My questions on Rust

1. **Does `println!` own data on heap? is it act as a `function`?**

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

2. **Access element from vec! and also push in same time?**
- `rust book` `listing 8-6`: Attempting to add an element to a vector while
  holding a reference to an item.

```rust
    let mut vec = vec![1, 2, 3, 4, 5];
    let first_element = &vec[0];

    vec.push(6);
    println!("The first element is : {:#?}", &vec[0]);

```


3. **Using enum with vec for &str**
- If you don’t know the exhaustive set of types a program will get at runtime
  to store in a vector, the enum technique won’t work. Instead, you can use a
  trait object, which we’ll cover in Chapter 17. ?  Chapter 8-9: enum with vec
  different types.


4. **Does `println!` borrow or own the variable?**
- As we already know, borrow means `&` a reference, while own means `become the
  owner` and stay in memory until get out of the scope.
 - The answer is, the `println!` and other similar macros (`print!`,
   `eprintln!`, `write!`, `format!`) are a special case and implciitly take a
   reference to any argumetns to pass to them. These macros do not behave as
   normal functions and macros do for reason of convenience,; the fact that
   they take referrence silently is part of that difference.
    - Read more [here](https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable)


5. **Why can i compare a String to a &str usign if , but not when using match?**
There is a mistamtch of types because `match` expression use patten matching, which is different from the `==` that are asssociated with `PartialEq` trait. There a secodn way to resovle this issue, by casting your `String` -> `&str` (a string slice):
```rust
let s: String = String::from("this is a heap allocated string .. ");

match &s[..]{
        "Holla!" => println!("It worked! ..."),
        "Hallo!" => println!("with easy to read matches! ..."),
        _ => println!("nothing ...")

    }
```
- [Reference](https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match)





