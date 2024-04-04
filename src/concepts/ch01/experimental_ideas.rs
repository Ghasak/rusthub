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
///
/// # Printing on same line
///
/// Args: No arguments
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

use crate::io::Result;
use crossterm::{cursor, style::Stylize, terminal, ExecutableCommand};
//use std::io::{stdout, Write};
pub fn over_write_console_output_enhanced() -> Result<()> {
    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout

    writeln!(
        stdout,
        "|{0:<25} | {1:<11} | {2:<10}",
        "URL Path", "Status Code", "Version"
    )?;
    writeln!(
        stdout,
        "|{0:<25} | {1:<11} | {2:<10}",
        "https://google.com", 200, 9
    )?;
    writeln!(
        stdout,
        "|{0:<25} | {1:<11} | {2:<10}",
        "https://yahoo.com", 200, 15
    )?;

    // wait 2 seconds before replacing lines
    std::thread::sleep(std::time::Duration::from_secs(2));

    stdout.execute(cursor::MoveUp(2))?;
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    writeln!(
        stdout,
        "|{0:<25} | {1:<11} | {2:<10}",
        "https://bing.com", 200, 3
    )?;
    writeln!(
        stdout,
        "|{0:<25} | {1:<11} | {2:<10}",
        "https://duckduckgo.com", 200, 1
    )?;

    Ok(())
}

///Another function used to print multi-lines on console
///
/// # multi-line overwriting the console
///
///
/// ## Args:
/// - `currently`: Not existed arg
///
/// ## Output:
/// - `simulation`: simulation points printed on the console, shows the performance progress.
///
/// ```rust
///
///    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout
///    for i in 0..100 {
///        writeln!(stdout,"")?;
///        writeln!(
///            stdout,
///            "|{0:<25} | {1:<11} | {2:<10} |",
///            "Maximum Likelihood", "Est.", "t-value"
///        )?;
///        writeln!(stdout,"")?;
///
/// ```
/// - [How to overwite multiple line in rust](https://stackoverflow.com/questions/72416445/how-to-overwrite-multiple-line-in-rust)

pub fn another_multi_line_console_cursor_output() -> Result<()> {
    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout
    for i in 0..100 {
        writeln!(
            stdout,
            "======================================================="
        )?;
        writeln!(
            stdout,
            "|{0:<25} | {1:<11} | {2:<10} |",
            "Maximum Likelihood", "Est.", "t-value"
        )?;
        writeln!(
            stdout,
            "======================================================="
        )?;

        writeln!(
            stdout,
            "|{0:<25} | {1:<11} | {2:<10} |",
            "LL(beta)",
            i,
            i + 1000
        )?;

        writeln!(
            stdout,
            "|{0:<25} | {1:<11} | {2:<10} |",
            "AIC",
            i,
            i + 100000
        )?;

        writeln!(
            stdout,
            "|{0:<25} | {1:<11} | {2:<10} |",
            "BIC",
            i * 2,
            i * 2 * 1000
        )?;

        writeln!(
            stdout,
            "======================================================="
        )?;

        // wait 2 seconds before replacing lines
        std::thread::sleep(std::time::Duration::from_millis(100));
        stdout.execute(cursor::MoveUp(7))?; // remember to move same number of writeln! occurence
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?; // can be ommited.
    }
    Ok(())
}

/// function used to generate random names
///
/// ## generate random emp names
///
/// We are using here the random generator to pick value from the list
/// Random choosing from a list
use rand::seq::SliceRandom;

use crate::concepts;

pub fn generate_emp_names(n: i32) {
    let first_name_list = ["Michael", "Jim", "Dwight", "Andy"];
    let last_name_list = ["Scouts", "Hibert", "Shrout", "Bernarnd"];

    for idx in 0..n {
        let first_name = first_name_list.choose(&mut rand::thread_rng());
        let last_name = last_name_list.choose(&mut rand::thread_rng());
        let combined_name = format!("{} {}", first_name.unwrap(), last_name.unwrap());
        println!("[+] [{}] Employee -> {}", idx, combined_name);
    }
}

/// Employee Random generator running function
pub fn executte_random_employee_generater() {
    use std::io;
    let mut n: String = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Something went wrong !!!!!");
    let num: i32 = n.trim().parse::<i32>().unwrap();

    println!("Generate number of employees {}... \n", num);
    generate_emp_names(num);
}
/// Implementing emojis and colors for console output
/// ## Emoji and color function
/// Args:
/// - No args, simple output in the console.
/// This function is used for demostration only
pub fn output_with_emoji_and_colors() {
    extern crate prettycli;
    use prettycli::*;
    // extern crate colored; // not needed in Rust 2018+
    // use colored::*;
    // use concepts::my_emoji;
    fn return_ref(name: String) -> String {
        let country = "Japan";
        let result = format!("{} added to {}", name, &country);
        result
    }
    let mycounter = return_ref("wow".to_string());
    info("We will print the number  ..."); //println!("Lets print the country: {}",mycounter );;
    println!("Lets print the country: {}", mycounter);

    for i in 0..10 {
        let temp_idx = format!("{i}");
        //println!("The value of i is -> {}".blue(), temp_idx);
        println!(
            "[{}] {}{} {} !",
            concepts::my_emoji("rocket"),
            "current value is =>".blue(),
            "it is ".green(),
            temp_idx.red().bold()
        );
    }
}
/// meaning of wrap_or function
///
/// ## wrap_or() meaning
///
///
/// ### Description:
/// Meaning of the `wrap_or` is like unwrap a value that we don't know, then once we open it, we give it a `value -> 0`
/// point to be taken for using the `match` for the `Result<T,E>` enum, is that both `T` and `E` must be same type.
pub fn meaning_of_wrap_or() {
    //extern crate colored; // not needed in Rust 2018+
    //use colored::*;
    let mut s: String = String::new();
    println!("How many points you want to create ... ? ");
    s.clear();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Maybe not working now.. ");

    // This is what the unwrap_or() function is doing
    // let output_num = match s.trim().parse::<i32>() {
    //     Ok(val) => val,
    //     Err(_)=> 0
    // };
    let output_num = s.trim().parse::<i32>().unwrap_or(0);

    for i in 0..output_num {
        println!(
            "[{}] {} {} {}",
            concepts::my_emoji("robot"),
            "wow ws".yellow(),
            "->".blue(),
            format!("{}", i).red()
        );
    }
}

/// printing nicely with spaces
///
/// ## Using Println! macro with sapces
///
///
/// ### Description:
/// The following function will print with spaces in the println! macro,
/// the syntax is like the str.formt synatx in python.
pub fn pretty_print_fn() {
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "total", "blanks", "comments", "code"
    );
    println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 0, 0, 0, 0);
    println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 77, 0, 3, 74);
    println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 112, 0, 6, 106);
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        460, 0, 10, 1371
    );
}
