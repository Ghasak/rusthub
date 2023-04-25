#![allow(clippy::let_and_return)]
use crate::colored;
use crate::concepts::ch01::common_collections;
use crate::concepts::ch01::experimental_ideas;
use crate::concepts::my_emoji;
use colored::Colorize;
use colored::*;

pub fn easy_rust_028_enums_part_1() {
    // sturct means AND something AND something ..etc.
    // Enum means OR soemthing, OR something, means only one choice at a time.

    #[derive(Debug)]
    enum ThingsInTheSky {
        Sun,
        Stars,
    }

    // println!(
    //     "Value of our selected value in this enum is : {:#?} -> \n and the type is : {}",
    //     ThingsInTheSky::Sun,

    //     common_collections::type_of(ThingsInTheSky::Sun)
    // );
    /// This is a greate function
    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Stars,
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => {
                println!(
                    "[{}] I can see the {} in the skye",
                    my_emoji("rocket"),
                    "sun".yellow()
                )
            }
            ThingsInTheSky::Stars => {
                println!(
                    "[{}] I can see the {} in the sky",
                    my_emoji("sparkles"),
                    "starts".blue()
                )
            }
        }
    }
    let mut current_time: String = String::new();
    let mut number_of_occurences: i32 = 0;
    loop {
        number_of_occurences += 1;
        if number_of_occurences > 10 {
            break;
        }

        println!("Please input the time of the day .. ");
        current_time.clear();
        std::io::stdin()
            .read_line(&mut current_time)
            .expect("We have input the current time");

        //let current_time_num = current_time.trim().parse::<i32>().unwrap_or(0);

        let current_time_num = match current_time.trim().parse::<i32>() {
            Ok(result) => result,
            Err(e) => 0,
        };
        let state: ThingsInTheSky = create_skystate(current_time_num);
        check_skystate(&state);
    }
}

pub fn enum_with_data() {
    enum ThingsInTheSky {
        Sun(String),
        Stars(String),
    }

    fn create_skystate(time: i32) -> ThingsInTheSky {
        let string1 = String::from("I can see the sun");
        let string2 = String::from("I can see the stars ...");
        match time {
            6..=18 => ThingsInTheSky::Sun(string1),
            _ => ThingsInTheSky::Stars(string2),
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun(description) => {
                println!("It is sunny ..., check the descrition: {}", description)
            }
            ThingsInTheSky::Stars(n) => println!(
                "Wow! there are stars in the sky ... check the decrition: {}",
                n
            ),
        }
    }

    let time = 9;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}

pub fn another_form_of_enum() {
    enum Mood {
        Happy,
        Sleepy,
        NoteBad,
        Angry,
    }

    fn match_mood(mood: &Mood) -> i32 {
        // you can also use the shortcut to import all the choices (variates) of the called enum
        use Mood::*;
        let happiness_level = match mood {
            // Mood::Happy => 10,
            // Mood::Sleepy => 6,
            // Mood::NoteBad => 7,
            // Mood::Angry => 2,
            Happy => 10,
            Sleepy => 6,
            NoteBad => 7,
            Angry => 2,
        };
        happiness_level
    }

    let my_mood = Mood::NoteBad;
    let happiness_level = match_mood(&my_mood);
    println!(
        "[{}] out of 1 to 10, my happiness level is : {}",
        my_emoji("rocket"),
        happiness_level
    );

    for i in 0..10 {
        println!(
            "[{}] out of 1 to 10, my happiness level is : {}",
            my_emoji("rocket"),
            happiness_level
        );
    }

    for i in 0..10 {
        println!(
            "[{}] The current value you are asking about is given as ",
            my_emoji("party_popper")
        );
    }
    for i in 0..100 {
        let idx: String = format!("{}", i);
        println!(
            "[{}] whatyou can make with the following: -> {} ",
            my_emoji("party_popper"),
            idx.red()
        );
    }
    #[derive(Debug)]
    enum my_enum {
        choice_1,
        choice_2,
        choice_3,
        choice_4,
        choice_5,
    }
}
