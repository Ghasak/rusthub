// use std::io ;
// use std::io::stdin; #![allow(clippy::assign_op_pattern)] #![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(rustdoc::invalid_rust_codeblocks)]
use std::io::{self, stdin, BufReader, Write};
use std::thread::sleep;
pub(crate) use std::time::Duration;

//use once_cell::sync::OnceCell;

mod concepts;
use self::concepts::ch01::enum_in_depth::enum_and_pattern_mathcing;
use concepts::ch01::{
    common_collections, enum_in_depth, error_handling_in_depth, experimental_ideas,
    memeory_investigating, ownership_borrowing, struct_in_depth_2, structs_in_depth,
};
use concepts::ch02::generic;
use concepts::ch03::{
    enum_in_rust, enum_option_in_depth, generic_in_rust, life_time_concept, results_and_options,
    rust_ownership_and_borrowing, structs_traits_and_implementation,
};

use concepts::ch03::compare_with_cpp::*;

use concepts::create_text;
use concepts::easy_rust::{
    self, easy_rust007_debug_printing, easy_rust008_mutability, easy_rust028_enums_all_parts,
};
use concepts::playground::{async_factorial_vs_sync_factorial, random_module, random_selection};
//? Out main function
///
//? # Main example library
///
//? ## Running Modules
/// Each module is nested all under [concepts]
// Example:
/// ```shell
///     cargo run --quiet
/// ```
/// =================== Decoration for the console output ====================
extern crate prettycli;
use prettycli::*;
extern crate colored; // not needed in Rust 2018+
use colored::*;
use concepts::my_emoji;
/// ==========================================================================
mod numerical_analysis;
use crate::concepts::ch03::results_and_options::MyInfo;
use numerical_analysis::ordinary_differential_equations;

fn main() {
    // create_text();
    // ownership_borrowing::about_owner_ship_concepts();
    // common_collections::common_collections_fn();
    //experimental_ideas::experiment_sum_fn();
    // memeory_investigating::investigate_memeory_allocation();
    // structs_in_depth::using_structs_to_structure_related_data();
    //experimental_ideas::over_write_console_output();
    //experimental_ideas::over_write_console_output_enhanced();
    //experimental_ideas::another_multi_line_console_cursor_output();
    // experimental_ideas::executte_random_employee_generater();
    //experimental_ideas::meaning_of_wrap_or();
    //experimental_ideas::pretty_print_fn();
    // enum_in_depth::enum_and_pattern_mathcing();
    //struct_in_depth_2::detecting_new_struct_initialization();
    //common_collections::hash_map();
    // error_handling_in_depth::error_handling_concept();
    // error_handling_in_depth::learning_about_error();
    //generic::learning_more_about_generics();
    // generic::working_with_generic();
    // generic::generic_function_example_01();
    // generic::generic_function_example_02();
    // generic::generic_function_example_03();
    // generic::generic_function_example_04();
    // easy_rust007_debug_printing::easy_rust_007_debuging_prinitng();
    //easy_rust008_mutability::eays_rust008_mutablility();
    //easy_rust028_enums_all_parts::easy_rust_028_enums_part_1();
    // easy_rust028_enums_all_parts::enum_with_data();
    //easy_rust028_enums_all_parts::another_form_of_enum();
    // ordinary_differential_equations::runge_kutta::welcome();
    //ordinary_differential_equations::runge_kutta::basis_of_ndarray();
    // ordinary_differential_equations::runge_kutta::welcome();
    // ordinary_differential_equations::runge_kutta::basics_of_ndarray();
    //ordinary_differential_equations::runge_kutta::welcome();
    //rust_ownership_and_borrowing::random_function();
    //random_module::random_stuff()
    //rust_ownership_and_borrowing::rust_owner_ship_and_borrowing_concept();
    //random_selection::using_random_selection_fn();
    //async_factorial_vs_sync_factorial::comparison_factorial_results();
    //rust_ownership_and_borrowing::rust_owner_ship_and_borrowing_concept();
    //enum_option_in_depth::option_enum_concept_fn();
    //life_time_concept::life_time_concept_fn();
    //structs_traits_and_implementation::rust_structs_traits_and_implementation_fn();
    //enum_in_rust::enum_in_rust_concept_fn();
    //generic_in_rust::generic_in_rust_concept();

    let v: MyInfo = MyInfo::new(
        "Jack".to_string(),
        "Michael".to_string(),
        true,
        23,
        "Jack_Michael@gmail.com".to_string(),
    );
    //results_and_options::results_and_option();

    let output = results_and_options::testing_my_info(v);
    let mut result = String::new();
    if let Some(v) = output {
        result.push_str(&v)
    }
    println!("{result:#?}");

    let my_string = String::from("This just a test for my current string .... ");
    println!("my current string is -> {my_string:#?}");
    println!("Trait Concept in depth ..");

    let logging_level: String = "ERROR".to_string();
    let varx: i32 = 10;
    println!(
        "[{}] Value of i -> {}",
        logging_level.yellow(),
        varx.to_string().red()
    );
    allocate_on_heap();
    let v = vec![1, 2, 3, 4, 5];
    let output = testing_function(&v);
    println!("The output is -> {:#?}", output);
    let v2 = vec![121.23, 433.53, 232.23, 1212.23];
    let o2 = my_function(&v2);

    let mut output = String::new();
    for i in 0..15 {
        let temp = format!("{}", i);
        output.push_str(&temp);
        println!("{i:#?} -> {output:#?}")
    }

    let arg1 = "Ok".to_string();
    let arg2: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    checking_perforamnce(&arg1, &arg2);

    let arg1 = String::from("INFO").yellow();
    let my_vector = vec![12.,322.,43.,1233.,122222.123];
    for item in my_vector{
        println!("[  {}  ] Item is  ... : {}", arg1, item);
    }






}

pub fn testing_function(my_vec: &Vec<i32>) -> Result<i32, String> {
    if my_vec.is_empty() {
        println!("The vector is empty -> ");
        Ok(0)
    } else {
        let my_string = String::from("Woww");
        Err(my_string)
    }
}

pub fn my_function(my_vector: &Vec<f32>) -> Option<String> {
    for i in my_vector {
        println!("The value of i -> {}", i);
    }
    let my_string = String::from("OK");
    Some(my_string)
}

pub fn my_current_file(my_vector: &[f32]) -> Result<f32, String> {
    if 1 > 2 {
        Ok(32.2)
    } else {
        Err("Wow".to_string())
    }
}

pub fn my_fucntion(my_vector: &str, my_another_vector: &[f32]) -> Option<String> {
    Some("This is just a testing".to_owned())
}

pub fn my_function_testing(
    string_param1: &String,
    vector_param2: &Vec<f32>,
) -> Result<String, String> {
    let mut idx: i32 = 0;
    if string_param1 == &"Ok".to_string() || string_param1.is_empty() {
        Ok("Yes the string is either OK or empty".to_owned())
    } else {
        for item in vector_param2 {
            idx = idx + 1;
            println!("The Item: {} is ->{}", idx, item)
        }
        Ok("The results are very much amazing...".to_owned())
    }
}

pub fn checking_perforamnce(param1: &str, param2: &Vec<f32>) -> Option<String> {
    if param1 == "OK" {
        Some("WOW".to_string())
    } else {
        let information_level = "INFO".to_string().green();
        for var in param2 {
            println!("[  {}  ] The value of i -> {}", information_level, var);
        }
        Some("Not Wow".to_string())
    }
}
