# Rust Life Time
- Difficult topic to learn
    - Even for seasoned developers.
- Summary of lifetime:
    - Each variable in Rust has a lifetime, which is the scope in which that
      variable is valid.
    - A lifetime is denoted by an apostrophe (') followed by a short name, like
      'a, 'b, etc.
    - A lifetime annotation is used to describe the relationships between the
      lifetimes of multiple variables. For example, if a reference to one
      variable is stored inside another variable, the lifetimes of the two
      variables must be related in some way.
    - Lifetime elision is a set of rules that allows the compiler to infer the
      lifetimes of references in many common cases. This can simplify code and
      reduce the need for explicit lifetime annotations.
    - The Rust borrow checker enforces the rules of lifetimes to prevent memory
      safety issues such as use-after-free errors.
    - The 'static lifetime refers to data that lives for the entire lifetime of
      the program. This is commonly used for string literals and other static
      data that is baked into the binary at compile time.

- [**GRule**]: It seems that we cannot return a reference for a variable created
  inside a funciton. As this variable will be dropped by reaching to the end of
  the function.

## Life time concept
- `lifetimes` are really about insuring memory doesn't get cleaned up before a
  reference can use it.
- Transformaing an ownership, is not a problem, but using reference, then you
  get this lifetime problem.
- `lifetime` is about references. and the references goes out of the scope.

```rust
let a;
{
    let b = String::from("Howday");

    a = &b;
}
println!("{a}" );
```
You, will get a complier error says:

```rust
`b` does not live long enough borrowed value does not live long enough.
```
- Notice that `a` is reference for nothing. as `b` is dropped.

### Example -1
- Missing Lifetime specifier this function's return type contains a borrowed
  value.but there is not value to be borrowed from.

```rust

fn get_int_ref()-> &i32{
    let a = 1;
    &a
}
```

- Rust complier knows, that since we didn't specifiy references in the param of
  the function.
- And, we created the reference internally (i.e., the borrowed reference would
  have to origiiinate from isndie the function), from the variable `a`.
- `a` will be dropped once its reach to the '}' of the end of the function,
  while the reference is return as `&i32`.
- Memeory gets cleanuped before the downstream code has a chance to use it.
- The life of the memeory we're returnign is not long enough if I return the
  ownership instead of reference, then it works as expected.


```rust
fn get_int_ref()-> i32{
        let a = 1
        a
    }
```
### Example -2

- Now,  we will introduce the `lifetime` specifier.
```rust
fn main(){
    let some_int_var : i32 = 10;
    let result_ref = get_int_ref(&some_int_var);
    println!("{result_ref}");
    }

fn get_int_ref(param_1: &i32)-> &i32{
    param_1
}
```
- Notice, that we don't have an issue with the function `get_int_ref` why?
- Rust can guarantee that the return reference will live long enough ifor downstream code to properly use it.
- Scope PROVIDING the reference is the same exact scope that will be RECEIVEING the result output.
- Meaning, that `param_1` reference scope is same as the return scope annotated with `-> &i32`.

### Example -3
Here, I do the same as above, but we will get very amazing idea which is
- Rust lifetime is to specify the life of the reference, but it will not
  enforce it, meaning, once the reference goes to end of the scope it will be
  dropped, even though we say that it should live like the input scope.

```rust

pub fn life_time_concept_fn() {

    let some_int_var : i32 = 10;
    let reference_int_val = &some_int_var;
    //let result_ref: &i32;
    {
       let  result_ref: &i32 = get_int_ref(reference_int_val );
    }
    println!("{result_ref}");

}

/// This function is used for testing purposes only.
fn get_int_ref<'a>(param_1: &'a i32)-> &'a i32{
    param_1
}
```

- You will get a complier error says `cannot find the value result_ref in this scope not found in this scope.`
- This means that `result_ref` is only valid within the inner block and does not
  exist outside of it. Therefore, when you try to print `result_ref` outside of
  the inner block using `println!("{result_ref}");`, the Rust compiler cannot
  find it in the current scope and throws an error.
- [**OPENAI-CHATGPT**] Rustlife time for reference is for specify how long the
  reference should live but not enforce it? right?
    - Yes, that's correct. Rust lifetime annotations are used to indicate to
      the compiler how long a reference should live. The Rust compiler will
      enforce these annotations and ensure that the lifetime of a reference is
      valid for the duration of its use. However, it does not guarantee that
      the reference will actually live that long, as it is ultimately up to the
      program's execution to determine the lifetime of the reference.
    - You can find more information about Rust lifetimes in the official Rust
      documentation. These resources provide detailed explanations, examples,
      and best practices for working with Rust lifetimes. Here are a few links
      that you may find helpful:
        - The Rust Programming Language book:
          https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
        - Rust by Example: https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html
        - Rustonomicon: https://doc.rust-lang.org/nomicon/lifetimes.html

- To fix the compliation error you can use
```rust

pub fn life_time_concept_fn() {

    let some_int_var : i32 = 10;
    let reference_int_val = &some_int_var;
    let result_ref: &i32;
    {
       result_ref = get_int_ref(reference_int_val );
    }
    println!("{result_ref}");

}

/// This function is used for testing purposes only.
fn get_int_ref<'a>(param_1: &'a i32)-> &'a i32{
    param_1
}
```

## Example -4
- The above example works, as we pass a `function` that return the reference
  with the `lifetime` specifier. We cannot compile the following code if its
  direct reference, as it will be considered as a dangling reference.
- The problem with this code is that lifetime_x is a reference to the value y,
  which is defined inside the inner block. When the inner block ends, y goes
  out of scope and its memory is deallocated. Therefore, lifetime_x becomes a
  dangling reference, pointing to memory that is no longer valid. When you try
  to print the value of lifetime_x outside the inner block, the compiler will
  raise an error because it is an invalid reference.

```rust

pub fn life_time_concept_fn() {
    let lifetime_x;
    {
        let y = 42;
        lifetime_x = &y;
        println!("{:?}", lifetime_x);
    }
    // lifetime_x is invalid here
    // let's print its value in the inner block instead
    println!("{:?}", lifetime_x);
}
```

## Example -5


```rust

```

