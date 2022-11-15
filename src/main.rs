// use std::io ;
// use std::io::stdin;

#![allow(clippy::assign_op_pattern)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(rustdoc::invalid_rust_codeblocks)]
use std::io::{self, stdin, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

//use once_cell::sync::OnceCell;

mod concepts;
use concepts::ch01::{
    common_collections, enum_in_depth, error_handling_in_depth, experimental_ideas,
    memeory_investigating, ownership_borrowing, struct_in_depth_2, structs_in_depth,
};
use concepts::create_text;

use self::concepts::ch01::enum_in_depth::enum_and_pattern_mathcing;
//? Out main function
///
//? # Main example library
///
//? ## Running Modules
/// Each module is nested all under [concepts]
// Example:
/// ```shell
///     cargo run --quiet
/// ````
fn main() {
    // create_text();
    // ownership_borrowing::about_owner_ship_concepts();
    // common_collections::common_collections_fn();
    //experimental_ideas::experiment_sum_fn();
    // memeory_investigating::investigate_memeory_allocation();
    // structs_in_depth::using_structs_to_structure_related_data();
    //experimental_ideas::over_write_console_output();
    //experimental_ideas::over_write_console_output_enhanced();
    experimental_ideas::another_multi_line_console_cursor_output();
    // enum_in_depth::enum_and_pattern_mathcing();
    // struct_in_depth_2::detecting_new_struct_initialization();
    //common_collections::hash_map();
    //error_handling_in_depth::error_handling_concept();

}

