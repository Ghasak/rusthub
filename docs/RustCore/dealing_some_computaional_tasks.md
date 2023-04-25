# Computational Tasks

The following is for some computational taks that I need to deal with and to
know more alternative for solving a given problem in `Rust`.

```rust
use std::io;
/// # Demonstration Function
/// ## Function Highlights
/// ### Input
/// The function require some inputs for demonstration.
pub fn life_time_concept_fn() {
    let output = examine_function(10.12, 121.21, 121.23);
    println!("{output:#?}");

    let input_vec: Vec<f64> = vec![121.32, 121.23, 22232.233, 121.44];
    let output = examine_function_2(&input_vec);
    println!("{output:#?}");

    let output: [f64; 4] = examine_function_3(&input_vec);
    println!("{output:#?}")


}
///Simple program show that we can create variable inside a function but must use ownership rule to return it
fn examine_function(param_a: f64, param_b: f64, param_c: f64) -> Vec<f64> {
    //let mut my_vec: Vec<f64> = Vec::new();
    let my_vec: Vec<f64> = vec![param_a, param_b, param_c];
    // my_vec.push(param_a);
    // my_vec.push(param_b);
    // my_vec.push(param_c);
    my_vec
}
///Simple program shows how to deal with creating a vector on Heap
fn examine_function_2(param_ve: &Vec<f64>) -> Vec<f64> {
    let mut output_vec: Vec<f64> = Vec::new();
    for item in param_ve {
        output_vec.push(*item);
    }
    output_vec
}

/// following function allocated on stack for the vector.
/// Vector cannot be change its size (adding more elements or delete elements)
///once its allocated on stack, but you can change the existed values stored already in this vector on stack
//fn examine_function_3(param_ve: &Vec<f64>) -> [f64; 4] {
fn examine_function_3(param_ve: &Vec<f64>) -> [f64; 4] {
    let mut output_stack_vec: [f64; 4] = [0.0; 4];
    for (idx, item) in param_ve.iter().take(4).enumerate() {
        output_stack_vec[idx] = *item
    }
    output_stack_vec
}
```
