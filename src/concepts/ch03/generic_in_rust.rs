use std::io;

use crate::concepts::ch03::structs_traits_and_implementation::RandomInfo;
use itertools::Itertools;

pub fn generic_in_rust_concept() {
    #[derive(Debug)]
    enum RandomInfo<T> {
        OptionA(T),
        OptionB(T),
        OptionC,
    }

    let my_data: RandomInfo<f32> = RandomInfo::OptionA(100.23);
    let my_other_data: RandomInfo<f64> = RandomInfo::OptionB(400.23);
    let last_data: RandomInfo<f32> = RandomInfo::OptionC;
    let some_option: RandomInfo<&str> = RandomInfo::OptionA("This is just  a test ...");
    let some_vec: RandomInfo<Vec<String>> =
        RandomInfo::OptionA(vec!["G".to_owned(), "H".to_owned()]);

    println!("{my_data:#?}");
    println!("{some_vec:#?}");

    let my_vec: [&str; 5] = ["1", "2", "3", "4.3", "aaa"];

    for val in my_vec {
        let output = val.parse::<i32>();
        println!("Currently parsing -> {val} into {output:#?}");
    }
    let output = another_fucntion_try(
        &"This is first senetence".to_owned(),
        &"This is second senetence".to_owned(),
    );
    println!("-------------------");
    println!("{output:#?}");
    println!("-------------------");
}

pub fn another_function(param_1: f32, param_2: f32) -> Option<String> {
    if param_2 != 0.00 {
        let output_message = format!("{} / {} = {}", param_1, param_2, param_1 / param_2);
        Some(output_message)
    } else {
        None
    }
}
/// This function is for testing purposes only
pub fn good_function(param_1: &str, param_2: &str) -> Option<String> {
    let mut my_output_string: String = String::new();
    if param_1 == param_2 {
        my_output_string.push_str("They are equal");
        Some(my_output_string)
    } else {
        my_output_string.push_str("They are not equal");
        Some(my_output_string)
    }
}

pub fn wow_function<'a>(param_a: &'a str, param_b: &'a str) -> Option<String> {
    let mut output_message: String = String::new();
    if param_a == "Yes" {
        output_message.push_str("Yes its going outside");
        Some(output_message)
    } else {
        output_message.push_str("I don't know what is this");
        Some(output_message)
    }
}

pub fn checking_error_type(param_1: &str, param_2: &str) -> Result<String, ()> {
    let mut result_output: String = String::new();
    if param_1.is_empty() {
        result_output.push_str("This is not working");
        Ok(result_output)
    } else {
        Err(())
    }
}

pub fn another_function_used_here(param_a: &str, param_b: &str) -> Result<String, String> {
    let mut output_message = String::new();
    let mut error_message = String::new();
    if param_a != param_b {
        output_message.push_str("They are equal!!");
        Ok(output_message)
    } else {
        error_message.push_str("They are not equal!");
        Err(error_message)
    }
}

pub fn another_fucntion_try(param_a: &String, param_b: &String) -> Result<f32, String> {
    if param_a != param_b {
        Err("they are not equal".to_string())
    } else {
        Ok(32.0)
    }
}

pub fn testing_function_alogrithm(param_a: &mut i32, param_b: &mut i32) -> Result<f32, String> {
    if param_a != param_b {
        let output_message = "Get me out of here ...".to_string();
        Err(output_message)
    } else {
        Ok(32.32)
    }
}
#[derive(Debug, Clone)]
pub struct myPrivateInformation {
    user_name: String,
    user_last_name: String,
    user_id: String,
    user_register_status: bool,
}
