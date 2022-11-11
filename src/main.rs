// use std::io ;
// use std::io::stdin;

#![allow(clippy::assign_op_pattern)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{self, stdin, BufReader};

mod concepts;
use concepts::ch01::{common_collections, experimental_ideas, ownership_borrowing, memeory_investigating};
use concepts::create_text;

fn main() {
    create_text();
    ownership_borrowing::about_owner_ship_concepts();
    common_collections::common_collections_fn();
    experimental_ideas::experiment_sum_fn();
    memeory_investigating::investigate_memeory_allocation();


}
