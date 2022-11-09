# Rust Fundamental programming

## String types
There are two types of strings in `Rust` and you can initialize the string on
`stack` or `heap`. The most important part, is that you have to compare two
strings should be same to produce the results you want.

- if the size is keep on changing use the `heap`
- if the size is stable and known in compilation time use `stack`
- You can append to string using `push` or `push_str` methods
- exclusively we distinguish between the two types as `&str` is on `stack`
  while `String` (`new`) keyword means its on `heap`.

## controlling memeory for variables
You can use `shadowing` which will allow you to keep same memory for a given
variable without a need to create new variable. This is useful when you want to
converate the same variable from one given type to another.


## Shadowing
One of the aspect, when using `loop` or `while`, is that if the `String` is keep on chaning, then you can shadow it at every iteration using


```rust
while some_value_bool {
    let mut guessing = String::new();
    }

```

## Empty String

```Rust
fn main() {
    let s1 = String::new();
    let s2 = String::from("");
    let s3 = "".to_string();
    let s4 = "".to_owned();
    let s5 = format!("");

    println!("({})", s1.trim().is_empty());
    println!("({})", s2.trim().is_empty());
    println!("({})", s3.trim().is_empty());
    println!("({})", s4.trim().is_empty());
    println!("({})", s5.trim().is_empty());
}
```


## Array and Vector in Rust
- `array` is what we call it a list, but must be all elements in same type. `array` is constructed on `stack` and must known the size.
- `Vector` is another list, but on `heap` the size can be different, all elements must be same data type.

```Rust
// Assumption
// guessing is String on heap created using String::new(); hold some string.


// 1. List on Stack can be written as:
let test_args: [&str; 3] = [ "quit", "Quit", "q"];
// On stack you can use, to compare &str to str  assume guessing is a String, trim will converate it from String to str
// 2. You can see that we derefernce the string to make it clear to compare &str to &str
let exiting_guessing = test_args.iter().any(|v| v == &guessing.trim());

// 3. If we want to find a string in a list of &str on stack
// let _existing_guessing = test_args.contains(&guessing.trim());

// *******************************************************************************************

// 1. List on Heap can be written as:
let test_args = vec!["quit".to_string(), "q".to_string(), "Q".to_string()];
// 2. On heap you can use, to compare String to String assume guessing is a String
let exiting_guessing = test_args.iter().any(|v| v == guessing.trim());
// 3. If we want to find a string in a list of string on heap
// let _existing_guessing = test_args.contains(&guessing.trim().to_string());

```
Lets check the code here


```rust
#![allow(clippy::op_ref)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //let test_args = vec!["quit".to_string(), "q".to_string(), "Q".to_string()];
    let test_args: [&str; 3] = ["quit", "Quit", "q"];
    for item in test_args.iter() {
        println!("item => {}", item);
    }
    let x = "q";
    let y = test_args.iter().any(|v| v == &x);
    println!("Yes we have found {}", y);

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is => {secret_num}");

    println!("Guessing Game for current object ... ");
    let mut _do_you_want_to_continue: bool = true;
    // You cannot get this one here, as it needed every trial in while .. <-
    //let mut guessing = String::new();

    let mut guessing = String::new();

    while _do_you_want_to_continue {
        println!("Please input your favorate number (guessing) ");
        // this will clear our String on the heap and make it ""
        guessing.clear();
        io::stdin()
            .read_line(&mut guessing)
            .expect("Error, expecting a user input");
        println!("You have input {}", &guessing);
        // if guessing.trim() == "q" {
        //     _do_you_want_to_continue = false;
        // }

        //let exiting_guessing = test_args.iter().any(|v| v == guessing.trim());
        let exiting_guessing = test_args.iter().any(|v| v == &guessing.trim());
        // you can also use ...
        // let _existing_guessing = test_args.contains(&guessing.trim().to_string());
        let _existing_guessing = test_args.contains(&guessing.trim());
        println!("The contains function doesnt accept trim, as it is required a String while trim produces a &str, currently the value is {}", _existing_guessing);
        if exiting_guessing {
            println!(
                "We will exit the program now, sinc eyou selelcted [{}]... ",
                &guessing.trim()
            );
            _do_you_want_to_continue = false;
        }

        //let guessing: u32 = guessing.trim().parse().expect("Please type a number!");
        let guessing: u32 = match guessing.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("**************************************************");
        match guessing.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //Ordering::Equal => println!("You win!"),
            Ordering::Equal => {
                println!("You win! ");
                break;
            }
        }
        println!("**************************************************");
    }
}

```




## Concepts
1.  Some()
2.  panic()
3.  wrap()
```Rust
// `unwrap` returns a `panic` when it receives a `None`.
//Example
let x = "32".parse::<i32>().unwrap();
```
4. casting
```rust
let another_temp_var = guessing.trim().parse::<i32>().unwrap();
// or
let another_temp_var: i32 = guessing.trim().parse().unwrap();

```



## Getting help and documentation
using
```shell
cargo doc --open
```


- [Programming idiom - syntax](https://programming-idioms.org/idiom/12/check-if-list-contains-a-value/414/rust)
- [Types of new string](https://www.hackertouch.com/how-to-create-and-check-string-is-empty-rust.html)
- [Accepting user input, remove trim](http://danielnill.com)
- [Compare between string literal &str vs String on heap ](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
