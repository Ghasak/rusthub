// use std::io ;
// use std::io::stdin;

#![allow(clippy::assign_op_pattern)]
#![allow(unused_imports)]
use std::io::{self, stdin, BufReader};

mod concepts;
use concepts::ch01::{common_collections, ownership_borrowing};
use concepts::create_text;

fn main() {
    println!("Hello, world!");
    create_text();

    ownership_borrowing::about_owner_ship_concepts();
    common_collections::common_collections_fn();

    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("maybe we can pass it .. ?");

    let s = s.trim().to_owned();

    println!("what is the value you input, is it {:#?}?", &s);
    println!("{}", common_collections::type_of(s));

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let nums = line
        .trim()
        .split(' ')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();
    let mut sum: f32 = 0.0;
    for num in nums {
        sum = sum + (num as f32);

        println!("{}", num);
    }
    println!("++++++++++++++++");
    println!("{}", sum);
}
