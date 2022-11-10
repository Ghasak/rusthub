// use std::io ;
// use std::io::stdin;

#![allow(clippy::assign_op_pattern)]
#![allow(unused_imports)]
use std::io::{self, stdin, BufReader};

mod concepts;
use concepts::ch01::{common_collections, ownership_borrowing, experimental_ideas};
use concepts::create_text;


fn main() {
    println!("Hello, world!");
    create_text();
    ownership_borrowing::about_owner_ship_concepts();
    common_collections::common_collections_fn();
    experimental_ideas::experiment_sum_fn();

}
