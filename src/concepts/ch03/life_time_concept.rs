use std::io;

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
    let variable_a: i32 = another_reference_dereference_fn_2(&mut variable_a);
    println!("value of <variable_a> after derefence and return the value -> {variable_a}");
}

pub fn changing_reference_dereference_fn_1(param_a: &mut i32) -> i32 {
    let param_b: i32 = *param_a + 10;
    *param_a += 10;
    param_b
}

pub fn another_reference_dereference_fn_2(param_a: &mut i32) -> i32 {
    *param_a += 10;
    *param_a
}
