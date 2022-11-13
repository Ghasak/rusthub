# Common Collections

**Notes**

- `i32` `u32` ..etc, are called `values implement the Copy trait`.
- `String`, `HashMap` ..etc. are called `owned value`

Basically we have:

1. a `vector` allows you to store a varaible number of values next to each
   other.
2. a `String` is a collection of characters. We've mentioned the `String`
   type previously, but this chapter will talke about it in depth.
3. a `has-map` allow you to associate a value with a particule key. It's a
   particular implmentation of the more general data structure called a map.

## Writing to vect

```rust
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

    let mut v : Vec<&str> = Vec::new();
    for i in 0..10{
        let x = format!("we have value -> {i}");
        let s: &'static str = super::common_collections::string_to_static_str(x);
        //let y:&str = &x[..]; <- this doesnt work
        v.push(s);

    }

    for item in &v{
        // you are passing here item which is a reference, (memory address), you need to dererence it at first.
        println!("We have now -> {}, with type -> {}", item, super::common_collections::type_of(*item));
    }



}

// This function will take String object and give us &str literal, It is implmentation of Rust 1.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
// this function will tell us the type of object in Rust
fn type_of<T>(_:T) -> & 'static str {
        std::any::type_name::<T>()
    }

```

## String - data collection

### Concatenation with the `+` Operator or the format! Macor

- Using `+` to concat two strings, in the form

```rust

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    println!("We have {} and {}", s1, s2);
    let s3 =  s1 + &s2;
    print!("now both have been concatented into -> [{}]", s3);


```

- you can use `format!`

```rust
format!("{}{}", s1, s2);
```

- Accessing `String` by index is not allow in `Rust`, for three reasons
- Since the string is a `utf-8` number of bytes per characters is not always matching.
- Bytes and scalar values and grapheme clusters!
- accessing should support O(1) but that not the case.

- Slicing strings: indexing into a string is often a bad idea beaxus its not clear what the return
  type of the string-indexign operation should be:
  - rather than indxin gusing `[]` with single number, you can use `[]` with
    a range to create a string slice containging particular bytes:
    ```rust
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    ```
  - be careful sometimes if you try to access some inidivudal character of
    utf-8 `String` you will get a compliation error. For the reasons that we
    stated above.
  - Best way is to iterate over each character in your `String`.
  ```rust
      for c in "Здравствуйте".chars(){
              println!("{}", c);
          }
  ```
  - Alternatively, the byte method returns `each raw byte` whic mgith be
    appropriate for your domain. But, be sure to remember that valid Unicode
    scalar values may be made up of more than 1 byte.
    ```rust
    for b in "Здравствуйте;".bytes(){
    println!("{}", b);
    }
    ```

## HashMap

Basically, there are two rules we have for the ownership:

- For types that implement the `Copy` trait, like `i32` the values are copied
  to the `hasmap`.
- For owned value like `String`, the values will be moved and the hash map will
  be the owner of those values, as demostrated in the follwoing example:

```rust

```

- If you store using the `clone()`, it means you have alraedy created a clone
  on the heap and given to the `has-map`,

  ```rust
  //map.insert(first_field.clone(), field_value.clone());
  ```

- If you will store `reference`, then you need to be sure that the
  `first_field`, and `field_value` should be both in the scope as long as the
  `has-map` is using them.

  ````rust

      let first_field = String::from("Favoriate color");
      let field_value = String::from("Blue");

      let mut map: HashMap<&String, &String> = HashMap::new();

      //map.insert(first_field.clone(), field_value.clone());
      map.insert(&first_field, &field_value);

      print!("{:#?}", first_field);


      ```

  ````

**Note** as you can see above that we annotiate the `map` using `<&String, &String>`, as we will store references not the real value.

### Create a generic function to print the HashMap

We will create a `generic` function that can accept any value, this function
will not affect the `ownership` as it is borrowing the hashmap and its value
all without taking ownership.

```rust

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

    let mut map : HashMap<&String, &i32> = HashMap::new();

    let c: String = String::from("keyAddress  -1- ");
    let d: String = String::from("keyAddress  -2- ");
    let e: String = String::from("keyAddress  -3- ");

    map.insert(&c,&1);
    map.insert(&d,&2);
    map.insert(&e,&3);


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

```


**Note**
- We can get the value of any key using the `API`
    ```rust

        //the api also bring us back the original value of the given key, if
        existed, and if not exist this key, it will be set to val
        let val = hasmap_name.entry(key).or_insert(val);

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


    ```
    ```shell
        ================ Note Important ========================
        Notice that we get the value of the key of &a, -> 10
        if key is not existed, then it will be setted to 1000 -> 1000
    ```
