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
