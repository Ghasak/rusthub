//use rand::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct my_struct {
    a: i32,
    b: f64,
    c: String,
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
pub fn rust_owner_ship_and_borrowing_concept() {
    let mut var_1 = my_struct { a: 9, b: 10.0 , c: "Wow".to_string()};
    some_procedure(&mut var_1);
    println!("{:#?}", var_1);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn some_procedure(my_strctu_param: &mut my_struct) {
    my_strctu_param.a = 155;
    println!("{:?}", my_strctu_param)
}
