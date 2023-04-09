// This is how you import from other modules in Rust
use colored::*;
use crate::concepts::my_emoji;

pub fn random_stuff(){

    for i  in 0..=10  {
        let item: String = format!("{i}");
        println!("[{}] The {} of our current file is given as {} {}",my_emoji("rocket"), "Analysis Value".yellow() ,"->".blue(), item.red());

    }

    for i in 0..=10{
        let s = format!("{i}");

        println!("[+] Current value of i = {s}")
    }

    for item in 0..=10{
        let item : String = format!("{item}");
        println!("[{}] We obtian the value of i -> at -> {}",my_emoji("rocket"),  item.red() );
    }

    #[derive(Debug)]
    struct Employee{
        user_name: String,
        age: i32,
    }

    let emp1 = Employee{
        user_name: "Transition Matrix".to_string(),
        age: 41,
    };
    for i in 0..=10{
        println!("-[+] You have here -> {}, {}", emp1.user_name, emp1.age);
    }
}
