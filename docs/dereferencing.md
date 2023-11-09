# De-referencing in Rust
#de-referencing #rust #compiler #reference

## Content
What is the meaning of `dereferencing` in Rust?
- De-referencing is a way to access the original variable in `Rust` meaning you
  are accessing the original value when it declared and any alternation will
  happen, it will affect the original value.
- It will access the original value and if you change the value it will change
  the original value (so you don't need to return the value from a function)
- It is useful when you have a variable that need to be increment (manipulated)
  and you don't want to create another variables inside the function to return
  them.

```rust
#[allow(dead_code)]
#[allow(unused_variables)]
/// # Demonstration Function
/// ## Function Highlights
/// ### Input
/// The function require some inputs for demonstration.
pub fn life_time_concept_fn() {

    let mut variable_a: i32 = 100;
    println!("**********************");
    println!("value of <variable_a> before derefence -> {variable_a}");
    changing_reference_dereference_fn_1(&mut variable_a);
    println!("**********************");
    println!("value of <variable_a> after derefence -> {variable_a}");

    println!("**********************");
    println!("Now we will get back the param from the function ");
    let variable_a : i32 = another_reference_dereference_fn_2(&mut variable_a);
    println!("value of <variable_a> after derefence and return the value -> {variable_a}");
}

pub fn changing_reference_dereference_fn_1(param_a: &mut i32) -> i32{
    let param_b : i32 = *param_a + 10;
    *param_a += 10;
    param_b
}

pub fn another_reference_dereference_fn_2(param_a: &mut i32) -> i32{
    *param_a +=10;
    *param_a
}


```

- Output of above code is

```sh
╰─ cargo run
   Compiling rusthub v0.1.0 (/Users/gmbp/Desktop/devCode/rust_fundamentals/rusthub)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/rusthub`
**********************
value of <variable_a> before derefence -> 100
**********************
value of <variable_a> after derefence -> 110
**********************
Now we will get back the param from the function
value of <variable_a> after derefence and return the value -> 120
```
