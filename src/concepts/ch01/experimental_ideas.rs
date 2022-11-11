#![allow(dead_code)]

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

pub fn experiment_sum_fn() {
    let mut s: String = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("maybe we can pass it .. ?");

    let s = s.trim().to_owned();

    println!("what is the value you input, is it {:#?}?", &s);
    println!("{}", super::common_collections::type_of(s));

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
/// This method is give use a indicator to be printed on same line
pub fn over_write_console_output() {
    let mut stdout = std::io::stdout();

    for i in 0..=100 {
        print!("\rEnter a number :=> {}%...", i);
        // or
        // stdout
        //     .write_all(format!("\rProcessing {}%...", i).as_bytes())
        //     .unwrap();

        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    println!();
}
