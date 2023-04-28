# Enum Option vs Result in Depth

Option and Result are both types in Rust that help handle situations where a
value may or may not exist or an operation may or may not succeed. The primary
difference between them is their use case. `Option` is used when a value may or
may not be present. For example, if you have a function that returns an
integer, but in some cases the result cannot be computed, you could use
`Option<i32>` as the return type. Option has two variants: `Some(T)` and `None`. When
you have a value, you return `Some(value)`. When you don't have a value, you
return `None`.
Result is used when an operation may or may not succeed. For
example, if you have a function that reads a file, it may succeed if the file
exists and can be read, but it may fail if the file does not exist or there is
a permissions issue. In this case, you could use `Result<String, io::Error>` as
the return type. Result has two variants: `Ok(T)` and `Err(E)`. When the operation
succeeds, you return `Ok(value)`. When the operation fails, you return
`Err(error)`.

In summary, use Option when you're dealing with the possibility of
a missing value, and use Result when you're dealing with the possibility of an
operation failing.

## Option Enum Concept

Typically, this involve using `Option` enum which is formulated from any data
structure. basically, its about if the value is available then it is `Some`
otherwise its `None`. Its a way to safely to get value form an operation if its
possible. There are several methods in rust to extract values from `OPTION`,
to name few:

- `unwrap()` method
- `expect()` method
- `match` statement
- `?`

```rust
pub enum Option<T>{
        Some(T),
        None
    }
```

### Example -1 How to use Option Enum

```rust
pub fn option_enum_concept_fn() {
    let short_vec : Vec<i32>= vec![1,2];
    let long_vec: Vec<i32> = vec![1,2,3,4,5];
    let my_output = take_fifth(short_vec);
    match my_output{
        Some(num) => println!("{num}"),
        None => println!("Cannot be extracted !!"),
    }
}

fn take_fifth(my_vector: Vec<i32>) -> Option<i32> {
    if my_vector.len() < 5 {
        None
    } else {
        Some(my_vector[4])
    }
}
```

### Example -2 Using Option Enum inside function

```rust

pub fn option_enum_concept_fn() {
    let short_vec : Vec<i32>= vec![1,2];
    let long_vec: Vec<i32> = vec![1,2,3,4,5];
    let my_output = get_value_by_index(&short_vec, 4);

    println!("{my_output}");
    println!("{:#?}", short_vec);

    #[derive(Debug)]
    enum my_option{
        Some(bool),
        None
    }

    impl std::fmt::Display for my_option{
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
            match self{
                my_option::Some(b) => write!(f,"Some({})", b),
                my_option::None => write!(f,"None"),
            }

    }
    }
    let my_option_true = my_option::Some(true);
    let my_option_false = my_option::Some(false);
    let my_option_none = my_option::None;
    println!("{}",my_option::Some(true));


}

fn get_value_by_index(my_vector: &Vec<i32>, get_element_idx: usize) -> String {
    match my_vector.get(get_element_idx){
        Some(num) => format!("{num}"),
        None => format!("The index you provided: {:#?} Cannot be extracted!!\nfrom the vector {:#?}",get_element_idx,my_vector),
    }
}
```

### Example -3 Find a word in a sentence.

```rust
pub fn option_enum_concept_fn() {
let sentence : &str = "This quick brown fox jumps over the lazy dog!";
let word : &str = "fox";

    match find_word(sentence, word){
        Some(index) => println!("Found the word '{}' at index {}", word, index),
        None => println!("The word '{}' was not found in the sentence", word),
    }


}


fn find_word(sentence: &str, word: &str) -> Option<usize>{
    if sentence.contains(word){
        Some(sentence.find(word).unwrap())

    }else{
        None
    }

}
```

### Example -4 Using Option vs Result

```rust

pub fn option_enum_concept_fn() {

    println!("{:#?}", divide_by(10.0, 20.0));
    println!("{:#?}", divide_by_2(10.0, 20.0));

    // applying the match on the option result
    let my_option:Option<f64> = divide_by(10.0, 0.0);
    match  my_option{
        Some(num) => println!("{num}"),
        None => println!("Cannot compute using Option ...")
    }

    // applying the match on the result
    let my_result: Result<f64, String> = divide_by_2(10.0, 0.0);
    match my_result{
        Ok(num) => println!("{num}"),
        error => println!("Cannot compute using Result ...")
    }


}

fn divide_by(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn divide_by_2(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero!"))
    }else{
        Ok(a / b)
    }
}
```

### Example -5 Using match with Option

```rust


let short_vector : Vec<i32> = vec![1,2,3];
let long_vector : Vec<i32> = vec![1,2,3,4,5,6];
let result1 = take_fifth(short_vector);
let result2 = take_fifth(long_vector );


//println!("{:#?}", result1);
//println!("{:#?}", result2);

// applying match
let output = match_value(result1);
 println!("{:#?}", output);
 match_value(result2);
}

fn match_value(result: Option<i32>) -> String{
    match result{
        Some(num) => format!("We get the value -> {num}"),
        None => String::from("We got no value, you must provide a longer vector .."),
    }
}

fn take_fifth(value: Vec<i32>) -> Option<i32>{
    if value.len() < 4 {
        None
    }
    else{
        Some(value[4])
    }
}
```

### Example 6- Another example

```rust

fn get_first_element(vector: Vec<i32>) -> Option<i32> {
    if vector.is_empty() {
        None
    } else {
        Some(vector[0])
    }
}

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector2 = Vec::<i32>::new();

    match get_first_element(vector1) {
        Some(element) => println!("The first element is: {}", element),
        None => println!("The vector is empty!"),
    }

    match get_first_element(vector2) {
        Some(element) => println!("The first element is: {}", element),
        None => println!("The vector is empty!"),
    }
}
```

## Result Enum Concept

\*\*

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Example -1 Using Result with reading file

```rust

use std::fs;

fn read_file(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(error) => Err(format!("Error reading file: {}", error)),
    }
}

fn main() {
    let file_path = "example.txt";
    match read_file(file_path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => println!("Error reading file:\n{}", error),
    }
}
```

### Example -2 Using Result enum with parsing

```rust

fn parse_int(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(String::from("Failed to parse integer!")),
    }
}

fn main() {
    let s1 = "123";
    let s2 = "abc";

    match parse_int(s1) {
        Ok(num) => println!("Parsed integer: {}", num),
        Err(err) => println!("{}", err),
    }

    match parse_int(s2) {
        Ok(num) => println!("Parsed integer: {}", num),
        Err(err) => println!("{}", err),
    }
}
```

### Example 3- Result vs If statement

```rust

pub fn option_enum_concept_fn() {

let my_vect : Vec<i32> = vec![1,2,3,4,5,6, 7];

if (my_vect.len() % 2) == 0 {
        println!("Yes, the vector is even ...")
    }
    else{
        println!("No, the vector is odd ..")
    }


let result : Result<_, String>= match my_vect.len() % 2{
    0 => Ok(()),
    _ => Err(String::from("The length of the vector is odd ..."))
    };
println!("{:#?}", result);
}


```

### Example 4- Split a vector with Result Enum

```rust

pub fn option_enum_concept_fn() {

let my_vect : Vec<i32> = vec![1,2,3,4,5,6, 7];

if (my_vect.len() % 2) == 0 {
        println!("Yes, the vector is even ...")
    }
    else{
        println!("No, the vector is odd ..")
    }


let result : Result<_, String>= match my_vect.len() % 2{
    0 => Ok(()),
    _ => Err(String::from("The length of the vector is odd ..."))
    };
println!("{:#?}", result);
let my_vect_f64: Vec<f64> = vec![0.1, 0.2, 0.3,0.4,0.5,0.6];
let output = extract_two_vectors_from_one(&my_vect_f64);
println!("{:#?}", output)


}



fn extract_two_vectors_from_one(my_vect: &Vec<f64>) -> Result<(Vec<f64>, Vec<f64>), String> {
    if my_vect.len() % 2 == 0 {
        println!("Yes, the vector is even...");
        let d: usize = my_vect.len() / 2;
        let half_vec1: Vec<f64> = my_vect[0..d].to_vec();
        let second_half_vec2: Vec<f64> = my_vect[d..my_vect.len()].to_vec();
        return Ok((half_vec1, second_half_vec2));
    }
    Err("The vector is not even.".to_string())
}

```
