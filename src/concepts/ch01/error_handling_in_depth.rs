#![allow(unused_must_use)]
pub fn error_handling_concept() {

    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    use std::io::{self, Read};
    /// Propagating Errors function, give you idea on how to handle the error
    /// this work only if your suername_file_result is given you Erro enum Reslt<T,E>
    fn read_username_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        println!("what is the is about -> {:#?}", username_file_result);

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    read_username_file();

    ///Shortcut for propgating errors: the ? Operator

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    //read_username_from_file();
    fn read_username_from_file_enhanced() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}

/// Recoverable error handling with propgating errors
///
/// ### RECOVERABLE ERRORS
///
/// Descritpion:
/// Errors that are meant to be handled are reutred with the `Result` enum.
pub fn learning_about_error() {
    fn ultimate_answer(guess: i64) -> Result<(), String> {
        if guess == 42 {
            return Ok(());
        }

        Err("Wrong answer".to_string())
    }
    // You can also alter the return value to be String
    fn ultimate_answer_enhanced(guess: i64) -> Result<String, String> {
        if guess == 42 {
            return Ok("The answer is correct:".to_string());
        }

        Err("This is wrong answer".to_string())
    }

    for i in 0..100 {
        let result = ultimate_answer_enhanced(i);
        if result.is_ok() {
            println!("We got the result => {:?} at {}", result, i);
        }
        else{
            println!("Error Message written to -> {:?} at {}",result, i ) ;
        }
    }
}



