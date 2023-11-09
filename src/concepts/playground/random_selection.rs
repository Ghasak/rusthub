//use rand::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]

pub fn using_random_selection_fn() {
    let stack_i8: i8 = 3;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    let my_stack_list: Vec<&str> = vec!["right", "left", "up", "down"];
    // Select randomly from a given list
    for idx in 0..3 {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..my_stack_list.len());
        let random_element = my_stack_list[random_index];
        println!("- [{idx}] => Random element: {random_element}");
    }

    // how to randomly select from enum

    #[derive(Debug)]
    pub enum Move {
        right,
        left,
        up,
        down,
    }

    impl From<usize> for Move {
        fn from(num: usize) -> Self {
            match num % 4 {
                0 => Move::right,
                1 => Move::left,
                2 => Move::up,
                _ => Move::down,
            }
        }
    }
    impl fmt::Display for Move {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Move::right => write!(f, "right"),
                Move::left => write!(f, "left"),
                Move::up => write!(f, "up"),
                Move::down => write!(f, "down"),
            }
        }
    }

    let mut rng = rand::thread_rng(); // Initialize a random number generator

    let mut input: String = String::new();
    println!("Enter the number of times to generate a random move:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let count: usize = input.trim().parse().expect("Invalid input");
    for _ in 0..count {
        let random_int = rng.gen_range(0..4); // Generate a random integer between 0 and 3
        let random_move: Move = random_int.into(); // Convert the integer to a Move enum value
        println!("Random move: {}", random_move);
    }
}
