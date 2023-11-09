use rand::random;

/// generic concept explained
///
/// ## Calling a generic
///
/// ### Description:
/// The current function introducing the concept of `generic`,
/// we refer to the type as `Thing` but usually it is a convention to call it just `T`
/// Example:
/// ```rust
///    fn return_thing<Thing>(thing: Thing) -> Thing {
///        println!("Here is your thing ... ");
///        thing
///    }
///
/// ```
pub fn learning_more_about_generics() {
    fn return_thing<Thing>(thing: Thing) -> Thing {
        println!("Here is your thing ... ");
        thing
    }

    let my_string = return_thing(String::from("I am a string allocated on the heap...."));
    println!("{}", my_string);
    let my_string = return_thing(7);
    println!("{}", my_string);

    use crate::concepts::ch01::common_collections;
    use std::fmt::{Debug, Display};

    fn another_generic_function<T>(input: T) -> T
    where
        T: Display + Debug,
    {
        println!(
            "We have applied the generic concept to {:#?} which is a type of -> {:#?} ",
            input,
            common_collections::type_of(&input)
        );
        input
    }

    let a = "this is just a string on the stack";
    let b = "this is another String allocated on the heap";
    let c = 10;

    another_generic_function(a);
    another_generic_function(b);
    another_generic_function(c);
}

pub fn working_with_generic() {
    use std::fmt::{Debug, Display};

    fn print_number<T>(number: T) -> T
    where
        T: Display + Debug,
    {
        println!("Here is your number: {}", number);
        number
    }

    print_number(8);
    print_number(10);
}

pub fn generic_function_example_01() {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Animal {
        name: String,
        age: u8,
    }

    fn print_item<T: Debug>(item: T) {
        println!("Here is your item: {:?}", item)
    }

    let charlie = Animal {
        name: String::from("Charlie"),
        age: 2,
    };

    print_item(charlie)
}

pub fn generic_function_example_02() {
    // More typing
    use std::cmp::PartialOrd;
    use std::fmt::Display;
    /// Function to implement two types of traits
    /// and utilize them to our input args
    fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
        println!(
            "{}! Is {} greater than {}? {}",
            statement,
            num_1,
            num_2,
            num_1 > num_2
        )
    }

    compare_and_display("Listen up! ", 9, 8);
}

pub fn generic_function_example_03() {
    use std::fmt::{Debug, Display};
    #[derive(Debug)]
    struct Animal {
        name: String,
        age: u8,
    }
    // More typing
    use std::cmp::PartialOrd;
    /// Function to implement two types of traits
    /// and utilize them to our input args
    fn compare_and_display<T: Display, U: Display + PartialOrd, V: Debug>(
        statement: T,
        num_1: U,
        num_2: U,
        animal: V,
    ) {
        println!(
            "{}! Is {} greater than {}? {}, and here we have animal -> {:#?}",
            statement,
            num_1,
            num_2,
            num_1 > num_2,
            animal
        )
    }

    let charile = Animal {
        name: String::from("Charile"),
        age: 10,
    };

    compare_and_display("Listen up! ", 9, 8, charile);
}
/// Implement Display to your struct
///
///
///
///
/// Description:
/// If you didnt implment the `Display` above, you cannot use it here in the generic
/// while the `Debug` trait is alreayd being used in the macro `Debug`.
/// For current implementation, the `strcut` has no `dervie` for `Display` similar to `Debug`, so instead, we need to implment that manually.
/// using:
/// ```rust
///    use std::fmt::{self,Debug, Display};
///    impl fmt::Display for Animal {
///        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///            write!(f, "({}, {})", self.name, self.age)
///        }
///    }
///
/// ```
pub fn generic_function_example_04() {
    /// If you want to implement a Display trait, you must hard-code it to your struct
    use std::fmt::{self, Debug, Display};

    #[derive(Debug)]
    struct Animal {
        name: String,
        age: u8,
    }

    impl fmt::Display for Animal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.name, self.age)
        }
    }

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 12,
    };
    // Notice that we don't need to specifiy the ? here
    println!("{}", charlie);

    // If you didnt implment the `Display` above, you cannot use it here in the generic
    // while the `Debug` trait is alreayd being used in the macro `Debug`.
    fn show_in_generic<T: Display + Debug>(animal: T) {
        println!("{}", animal);
    }

    show_in_generic(charlie);
}
