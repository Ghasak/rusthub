// use std::io ;
// use std::io::stdin;

#![allow(clippy::assign_op_pattern)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{self, stdin, BufReader};

mod concepts;
use concepts::ch01::{common_collections, experimental_ideas, ownership_borrowing};
use concepts::create_text;

fn main() {
    create_text();
    ownership_borrowing::about_owner_ship_concepts();
    common_collections::common_collections_fn();
    experimental_ideas::experiment_sum_fn();

    let ptr: u8 = 255; // 0000-0000<->0000-0000<->0000-0000<->0000-0000 <- maximum number per byte is {255: decimal} or {ff: hex} or {1111-1111:binary}
                       // method only for float
    println!(
        "value: {}, \nByte: {:b}\nMemory Address: {:p}\nHexadecimal: {:x}",
        ptr, ptr, &ptr, ptr
    );
    // let ptr: &str = "a";
    // println!("{:?}", ptr.as_ptr());

    // let my_value = 123;
    // // get a reference to the value
    // let my_value_ref = &my_value;
    // // convert the reference to a raw pointer
    // let my_value_raw_ptr = my_value_ref as *const i32;
    // // convert the raw pointer to an integer
    // let my_value_addr = my_value_raw_ptr as usize;
    //
    // println!("address = {:X}", my_value_addr);
}
