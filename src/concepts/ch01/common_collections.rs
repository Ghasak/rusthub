#![allow(unused_variables)]

pub fn common_collections_fn() {
    // create a new vector

    let mut v: Vec<String> = Vec::new();
    //let adding:String = String::new();

    for i in 0..4 {
        //println!("value of i = {i}");

        let my_string = format!("value of i => {}", i);

        v.push(my_string);

        // let x : i32 = i;
        // v.push(x);
        let temp_stack_string: String = format!("we are making now => {}", i);
        // or temp_stack_string.as_mut_str();
        let temp_stack_string = temp_stack_string.to_ascii_uppercase();
        println!("{}", temp_stack_string);
    }

    println!("Lets examine the v: {:#?}", v);

    println!("==============================================",);

    let mut vv = Vec::new();
    // Write to a vect
    for i in 0..=9 {
        vv.push(i);
    }
    println!(
        "Our vector is now populated with range from 0 -> 9 and 9 is included {:#?}",
        vv
    );
    println!("==============================================",);

    // Read from a vect

    let third: &i32 = &vv[2];
    println!("The third element is {}", third);
    // or to be on safe side, in case we want to check, if there is value at this index

    let third: Option<&i32> = vv.get(2);
    match third {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element ..."),
    }

    let super_big: Option<&i32> = vv.get(10);
    match super_big {
        Some(super_big) => println!("We have found this element"),
        None => println!("No ..... ohh we are out of range ..."),
    }

    println!("the value of super_big is -> {:#?}", super_big);

    //let another_variable: &i32 = vv.get(100).unwrap();
    //println!("Using unwrap to see if the value is existed ... otherwise it will panic {}",another_variable);

    let x: String = String::from("this is just a string ... allocated on the heap");

    let y: Option<String> = Some(String::from(
        "this is my -> y <- string ... allocated on the heap as well",
    ));

    match &y {
        Some(s) => println!("Yes it's exited and it is => {:#?}", s),
        None => println!("No it's exited and it is => None",),
    }

    println!("what is the value of right now ... ? {:#?}", y);
    println!("what is the value of right now ... ? {:#?}", y);
    println!("what is the value of right now ... ? {:#?}", y);
    println!("==============================================",);
    println!("[!] Listing `rust book` `listin 8-6`: Attempting to add an element to a vector while holding a reference to an item.",);
    println!("==============================================",);

    let mut vec = vec![1, 2, 3, 4, 5];
    let first_element = &vec[0];

    vec.push(6);
    // if you use now first_element, it will panic and not allow, as the first_element, has been changed, since you push new element (6)
    println!("The first element is : {:#?}", &vec[0]);

    println!("==============================================",);
    println!(" Using an Enum to Store Multiple Types ..",);
    println!("==============================================",);

    #[derive(Debug)]
    enum SpeardsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpeardsheetCell::Int(10),
        SpeardsheetCell::Float(10.0),
        SpeardsheetCell::Text("Wow, it can store string also".to_string()),
    ];

    println!(
        "Our new collection that store several types is = {:#?}",
        row
    );

    println!("==============================================",);
    println!(" Storing UTF-8 Encoded Text with Strings",);
    println!("==============================================",);

    let s1 = "this is a string literal ".to_string();
    let s2 = &s1[..];

    let mut v: Vec<&str> = Vec::new();
    for i in 0..10 {
        let x = format!("we have value -> {i}");
        let s: &'static str = super::common_collections::string_to_static_str(x);
        //let y:&str = &x[..]; <- this doesnt work
        v.push(s);
    }

    for item in &v {
        // you are passing here item which is a reference, (memory address), you need to dererence it at first.
        println!(
            "We have now -> {}, with type -> {}",
            item,
            super::common_collections::type_of(*item)
        );
    }

    println!("==============================================",);
    println!(" Concatenation with the `+` Operator or the format! Macor",);
    println!("==============================================",);
    // Concatenation with the `+` Operator or the format! Macor

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    println!("We have {} and {}", &s1, &s2);
    let s3 = s1.to_string() + &s2;
    print!("now both have been concatented into -> [{}]", s3);

    let new_string = format!("->>{}{}<<-", s1, s2);
    println!("\n");
    println!("{}", new_string);

    println!("==============================================",);
    println!("Slicing String!!!!",);
    println!("==============================================",);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("what we want to pritn is : {}", s);

    println!("==============================================",);
    println!("Method for iterating over strings .. ",);
    println!("==============================================",);
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // Alternatively, the byte method returns `each raw byte` whic mgith be appropriate for your domain.
    for b in "Здравствуйте;".bytes() {
        println!("{}", b);
    }
    //match comparision between heap allocated String vs stack string &str (slice &str)
    // notice that &s[..] give us a &str (string literal which is on the stack)
    let s: String = String::from("this is a heap allocated string .. ");
    match &s[..] {
        "Holla!" => println!("It worked! ..."),
        "Hallo!" => println!("with easy to read matches! ..."),
        _ => println!("nothing ..."),
    }
}

// This function will take String object and give us &str literal, It is implmentation of Rust 1.
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
// this function will tell us the type of object in Rust
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

/// This function will tell us eveything about hashtable ...
pub fn hash_map() {
    println!("The general form is: HashMap<K,V>, Chapter -8- 8.3",);
    println!("Storign keys with Associated Values in Hash Maps ..",);

    use std::collections::HashMap;

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for item in scores.iter() {
        println!(
            "Printing the item object consists of key and value {:#?}",
            item
        )
    }

    for (key, value) in scores.iter() {
        println!("our value is => key: {:#?}, value: {:#?}", key, value);
    }

    // Accessing sepcific item form our hash_map

    let searching_key = String::from("blue");

    // notice now that the score is own the value that we copied form the hash_map
    let score = scores.get(&searching_key).copied().unwrap_or(0);
    println!("Value of score for {} is -> {}", searching_key, score);

    // Hash Maps and OwnerShip
    // For types that implemetn the `Copy` trait, like `i32`, the values are
    // copied into the hash map. For owned vlaues like `String`, the values will
    // be moved and the hash map will be the owner ofthose values, as
    // demonstrted in the

    let first_field = String::from("Favoriate color");
    let field_value = String::from("Blue");

    let mut map: HashMap<&String, &String> = HashMap::new();

    //map.insert(first_field.clone(), field_value.clone());
    map.insert(&first_field, &field_value);

    print!("{:#?}", first_field);
    println!("\n========================================================",);
    println!("       overwriting a value                         ",);
    println!("\n========================================================",);

    let a = String::from("Wow");
    let b = String::from("can we do that");
    map.insert(&a, &b);

    ///This function can print value-key for hashmap
    // It is implemented for a type key:`&String`-value:`&String`
    fn fancy_hash_map_fn(my_hash_map: &HashMap<&String, &String>) {
        for (key, value) in my_hash_map.iter() {
            println!("our value is => key: {:?}, value: {:?}", key, value);
        }
    }

    fancy_hash_map_fn(&map);

    println!("\n========= We will overwrite the map ===================",);
    use std::fmt::Debug;
    use std::fmt::Display;
    /// This function is generic and be used to any value and any type, to print their values
    fn fancy_hash_map_fn_generic<T: Display + Debug, U: Display + Debug>(
        my_hash_map: &HashMap<&T, &U>,
    ) {
        for (key, value) in my_hash_map.iter() {
            println!("our value is => key: {:#?}, value: {:#?}", *key, *value);
        }
    }
    // Examples ---

    let mut map: HashMap<&String, &i32> = HashMap::new();

    let c: String = String::from("keyAddress  -1- ");
    let d: String = String::from("keyAddress  -2- ");
    let e: String = String::from("keyAddress  -3- ");

    map.insert(&c, &1);
    map.insert(&d, &2);
    map.insert(&e, &3);

    fancy_hash_map_fn_generic(&map);
    /// Fancy function to print the items of the hash_map, it will not affecting the ownership
    /// Argument: HashMap --> <&T,&U>
    fn fancy_hash_map_fn_generic_enhanced<T, U>(my_hash_map: &HashMap<&T, &U>)
    where
        T: Display + Debug,
        U: Display + Debug,
    {
        for (key, value) in my_hash_map.iter() {
            println!("our value is => key: {:#?}, value: {:#?}", *key, *value);
        }
    }

    println!("========================================================",);
    println!(" adding a key and value only if a key isn't present",);
    println!("========================================================",);

    println!("We are using the method: entry",);
    //clearer trait bounds with `where` clauses

    let mut scores = HashMap::new();
    let a = String::from("Blue");
    let b = String::from("Yellow");
    let c = String::from("Blue");
    let d = String::from("Red");

    scores.insert(&a, &10);
    scores.entry(&a).or_insert(&50);
    scores.entry(&b).or_insert(&50);
    scores.entry(&d).or_insert(&100);
    let w = String::from("wow");

    println!("================ Note Important ========================",);
    println!("Notice that we get the value of the key of &a, -> {:#?}\n if key is not existed, then it will be setted to 1000 -> {:#?}", scores.entry(&a).or_insert(&0).clone(), scores.entry(&w).or_insert(&1000));
    println!("========================================================",);
    fancy_hash_map_fn_generic_enhanced(&scores);

    println!("========================================================",);
    println!("    Update a value base don the old value ",);
    println!("========================================================",);

    let text = "Hello world wornderful world ... Can we see if we got the world as we expected, wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        let our_type = super::common_collections::type_of(&count);
        *count += 1;
        println!(
            "value of count: word::{} -> count::{} -> type::{}",
            word, count, &our_type
        );
    }
    println!("{:?}", map);
}
