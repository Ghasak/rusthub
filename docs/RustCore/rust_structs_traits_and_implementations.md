# Rust structs, Traits, and Implementation

## Introduction

- Based on lecture notes by `Doug Milford` with my notes and experiments.

## Concepts

### RUST Struct Topics

- `Rust struct` or `strctures` are similar to classes or objects in other
  languages with notable distictions.
  - `struct` reprsents complex data types that you define.
  - For those who are coming from an object-oriented world, `struct` are like
    `object`, but has differences. Making them object-orientd-ish. They don't
    quite fit the true definition. We will explosre how they're similar but
    different for one:
    - `Rust` doesn't have inheritance,
    - but it can have methods.
    - `Rust` also has `Traits` - Similar to polymorphism for object oriented.
      which is a certain abstract behavior similar to what polymorphism does in
      object-oriented languages. but, since there is no inheritance in `Rust`
      it's not quite the same.
  - Derived traits can be done using macros.

---

- Struct form is constructed by

```rust
struct Mystruct{
        field_1: type
        field_2: type
        .
        .
    }
```

- Lets experiment with some `struct`, lets define a sturct (hopfully with a
  name descritive enough for other users to know what they represent)
  - There is a (comma `,`) among the defined fields, also last field has a
    comma as well.
  - Organizing fields of hundards of them can be done in several ways,
    sometimes by importance, sometimes by use or type. Best way to organize them alphanically.

```rust
struct Mystruct {
        some_bool: bool,
        some_float : f32,
        some_int : i32,
    }

fn main(){
        let my_obj = Mystruct{
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            };

    }
```

- Access a field from our created object is simple `my_obj.some_bool` if you
  want to change the value, `Rust` by default create the `struct` immutable.
  So, we must specify the struct has to be mutiable.

```rust
        let mut my_obj = Mystruct{
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            };

        my_obj.some_bool = false;  //<- now can be accepted.
```

- We just need to tell only the variable which one is `mut` and all variables
  will follow to be mutiable as well.

- Here, we will construct a new similar struct type variable based on existed
  struct variablewe use the other fields from the my_obj strcut object to
  construct the remining fields. it says for `another_obj` that we will set the
  field with value `200` while reset of fields all of the obj `my_obj`

```rust

        let mut my_obj = Mystruct{
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            };
        let another_obj = Mystruct{
                some_int : 200,
                ..my_obj //<- we use the other fields from the my_obj strcut object to construct the remining fields.
            }

```

- Now we speak about `struct` visibility. `Rust` assumes all fields are not
  visible when you call them from a different `mod` `file` whatever. Using
  `mod` we will import the struct to our main.rs file. We also add the (use)
  that when we're referencing struct, we don't need to specify the mod name
  everytime

```rust
// we have a file named random_info_file.rs as
struct RandomInfo{
        some_bool: bool,
        some_int : i64,
    }
// within same directory of the file above we will have our main.rs

mod random_info_file;
//
use random_info_file::*;
```

- Now, yrou notice that we need to specify `pub` for the struct

```rust

// we have a file named random_info_file.rs as
pub struct RandomInfo{
        pub some_bool: bool, //<- only this will be public
        some_int : i64,      //<- will not be visible
    }
// within same directory of the file above we will have our main.rs
mod random_info_file;
use random_info_file::*;
```

- In summary, Import a struct form another visibility, `Rust` assume for
  visibility everything is private.

| note       | Rule                                               | keyword | keyword location |
| ---------- | -------------------------------------------------- | ------- | ---------------- |
| Mutability | apply to all fields infront the object name        | mut     | on object        |
| visibility | apply field by field infornt the struct definition | pub     | on definition    |

- `Rust` team embaraces composition over `inhertence` so, no `inheritance` in
  `Rust`. Here the RandomInfo is a struct from the (random_info) module used in
  composition to MyStruct. whether inherence or composition is better there is
  a hot disucssion on both, its a bit of philosophical disucssion. In `Rust`
  you cannot `inherit` so it's kind of a moot point.

```rust
mod random_info;
use random_info::*;

#[allow(dead_code)]
struct MyStruct{
        some_bool: bool,
        some_float: f64,
        some_int: i32,
        random:RandomInfo, //<- Here the RandomInfo is a struct from the (random_info) module used in composition to MyStruct
    }
```

- You can intialize the data for the `struct composition` as shown below. Its
  is a nice seque into how to do methods similar to object-oriented programming
  as well as introducting `traits`.

```rust
let mut obj = MyStruct{
        some_bool : true,
        some_float: 10.3,
        some_int: 80,
        random: RandomInfo{
                some_bool: false,
                some_int: 90,
            }
    }
```

- Associate `methods` to `struct` using the `impl` keyword. They're assoicated
  with a struct type you use the `impl`keyword meaning implement or
  Implementation followed by the type you implementing for. Inside of that area
  is where you put functions and methods.

```rust
    #[derive(Debug)]
    pub struct RandomInfo {
        pub some_bool: bool,
        pub some_int: i32,
    }
    impl RandomInfo {
        pub fn new(param_a: bool, param_b: i32) -> RandomInfo  {
            RandomInfo {
                some_bool: param_a,
                some_int: param_b,
            }
        }
    }

    let my_obj = RandomInfo::new(true, 12);
    println!("{my_obj:#?}")
```

- Notice that you can write also with `Self` keyword instead hardcoded `struct`
  name. The convention is to use the `Self` keyword the complier knows is the
  `RandomInfo` strut type that you're implementing for. It helps keep things
  abstract and if you see it in someone else's code you shoud know what it
  means. `Self` capitalization is important and `Self` represent the type so
  first letter must be captilized. Now, you can code to your heart's content to
  prep any data needed to create your return variable. It is just like any
  other function. Remember, `Rust` knows that you will return `Self` when you
  omit your semicolon. All it means, I'm reaturning a type of self which the
  complier knows this is a `RandomInfo` struct type and then return it

```rust

pub fn new(param_a: bool, param_b: i32) -> Self {
    RandomInfo {
        some_bool: param_a,
        some_int: param_b,
    }
// Or you can even write [Personally I use this]
pub fn new(param_a: bool, param_b: i32) -> Self {
    Self{
        some_bool: param_a,
        some_int: param_b,
    }

```

- Lets add another `method` that uses an actual data in the `RandomInfo`
  struct. This time we will use the `&self` parameter to any `method` which is
  created in our struct Implementation. In this case, it's all lowercase with
  `&` in front of it. This is different from `Self` captilized. `Self` as we
  stated eariler, its a type, while the `self` lowercase means it represents
  actual data. This is how you can access and modify the data of your struct
  internally.

```rust
pub fn rust_structs_traits_and_implementation_fn() {
    #[derive(Debug)]
    pub struct RandomInfo {
        pub some_bool: bool,
        pub some_int: i64,
    }
    impl RandomInfo {
        pub fn new(param_a: bool) -> Self {
            Self {
                some_bool: !param_a,
                some_int: 8,
            }
        }
        pub fn is_smaller(&self, compare_to: i64) -> bool {
            self.some_int < compare_to
        }
    }
}
```

- When you use `Self` for a method implmented for a struct then you don't need
  to pass `&self` and you can access it using `::` it works like a constructor
  to make the object. Means the method is accessing the type itself not the object.
- If you want to read/write to a variable inside the struct, any method
  associated with the data and not a constructor then it needs the `&self` in
  its signature. Also can access the data using the `.` dot notation.Means the
  method is accessing the data of the object not the type.

- If we want to modify a data, then `Rust` assumes immuntablility to all the
  elements of the struct data. We must tell `Rust` for the method that
  implemented with `&self` that we mut the data by offering the `mut` in the
  signature of the implmented method. We also need to add `mut` to the
  constructed object `my_obj` thats why you see we added `mut` to it. Now
  everything works, this chasing down errors and warnings is considered to be a
  nuisance by many people learning rust for first time. It's the opposite of
  the loosey goosey `JavaScript`. The complier can be somewhat annoying at
  first to do `A`, `B` and `C` , but `Rust` ensure that we can do from `A` to
  `Z`. The strictness saves you time because errors and debugging time plummet,
  which is really where projects get bogged down.

```rust

pub fn rust_structs_traits_and_implementation_fn() {
    #[derive(Debug)]
    pub struct RandomInfo {
        pub call_count: i64,
        pub some_bool: bool,
        pub some_int: i64,
    }
    impl RandomInfo {
        pub fn new(param_a: bool) -> Self {
            Self {
                call_count: 0,
                some_bool: !param_a,
                some_int: 8,
            }
        }
        pub fn is_smaller(&mut self, compare_to: i64) -> bool {
            self.call_count += 1;
            self.some_int < compare_to
        }
    }

    let compare_to_var = 100;
    let mut my_obj = RandomInfo::new(true);
    println!("{my_obj:#?}");
    let output = my_obj.is_smaller(compare_to_var);
    println!("{output:#?}");
    println!("{:#?}", my_obj.call_count);
}
```

- Anyway, I'm side tracking, one valid question is the Implementation of
  function and methods outside the struct definition. One reason, and perhaps
  there's more is to augment someone else's struct. In object-oriented
  programming you might inherrit from a base class and then add more
  functionality. In `Rust` you can just implement anywhere you like. Let's do a
  different implementation for a function called is larger, notice that we
  don't need to go to the file where the struct or other methos are located.
  Instead we simply just say `impl struct_name` anywhere in the program. Event
  though struct it wasn't defined in the struct definition.

```rust
impl RandomInfo{
        pub fn is_larger(&self, compare_to: i64) -> bool{
                self.some_int > compare_to
            }
}
```

- So, It doesn't matter if you're using somebody else struct buried in some
  reference crate. You can extend it quite easily.

---

| idx | Note                                                                                            | Example                                   |
| --- | ----------------------------------------------------------------------------------------------- | ----------------------------------------- |
| 1.  | It seems `new` function name is the only method for impl for a struct, doesn't need `&str`      | fn new(param_a) almost like a constructor |
| 2.  | You must provide `&self` for every function for any implmented method for the struct,           | fn some_method(&self, param_a)            |
| 3.  | You need `Self` to refere to the `struct` and it is considered as a `type` accessed with (`::`) | let obj = RandomInfo::new()               |
| 4.  | You will need `&self` for accessing data in the constructed `struct` access with (`.`) notation | obj.data_value                            |

---

- Object-oriented programming also has a concept of polymorphism. Meaning
  treating different objects the same regardless of whether they're inherited
  or the base type. `Rust` handle polymorphism a little differently with
  something called traits.

- Lets create a trait called SomeTrait. Its almost same as writing `methods` in
  struct impmentation, except that we don't actually write the body we just put
  a `semicolo` after the type. Notice, why the `trait` uses `&self` can make
  more senence now, since the trait can be applied on an indefinite number of
  structs, it wouldn't be feasible or perhaps possible for create a distanct
  function signature for each one, but with the `&self` keyword it doesn't
  matter. It's a way for us to be more abstract about our programming.

```rust
pub trait SomeTrait{
    fn is_valid(&self)-> bool;


    }
```

- In line with that let's say you wanted to implement a function that compared
  a different variables of the same type. btw, We use `Pascal's` cases to write
  the `Struct` and the `Trait` names.

- Lets use our single function `is_valid` on our `RandomInfo` struct. When you
  impement a `Trait` you don't need to put whether it's public or not, It's
  just implied to be public. In fact, It will give you a complie error if you
  want to put the keyword `pub`.

```rust

pub trait SomeTrait{
    fn is_valid(&self)-> bool;
    }

impl SomeTrait for RandomInfo{
        fn is_valid(&self) -> bool {
                self.some_bool
            }
    }
```

- Let's go a head and use our newly fulfilled trait. You may be wondering what
  the trait even bought us it was so similar to how we dfine other functions so
  what's the point?. Well, a `Trait` pretty much decouples the function
  definitions so that can be used on multiple structs this is how `Rust`
  achieve polymorphism.

- Lets check about adding some functionality to a struct without a need to know
  where the struct is located.
-

```rust
impl SomeTrait for RandomInfo{
        fn is_valid(&self) -> bool {
                tru
            }
    }

fn print_if_is_valid(check_me: &dyn SomeTrait){
        if check_me.is_valid(){
                println!("Yay!");

            }
    }

```

- Let's implement a `trait` called `Default` which will allow to initalize data
  for our created `struct`. These data will be the default once you constrcut your struct `MyStruct`.

```rust
impl Default for MyStruct{
        fn default() -> Self{
                Self{
                    some_bool : true,
                    some_float: 10.3,
                    some_int : 80,
                    random::RandomInfo::new(true),
                    }
            }
    }
```

- Now, If you need a default data, you can change,

```rust
let my_obj = MyStruct{
        some_bool : true ,
        some_float: 10.3,
        some_int : 80,
        random: RandomInfo::new(true),
    }

```

- To the following

```rust
let my_obj = MyStruct::default();
```

- Often you will see a `trait` called `Debug` which is often used when printing
  to the terminal or a formatted string. You get a complie error because we
  aren't implementing trait debug yet, Instead implementing the `Debug` trait
  by hand, we can use `Macro` to implement that trait to our defined strcut.
  We just annotate our `struct` with `#[derive(Debug)]`.
- You can also import `traits` which are very powerful like `Copy` `Clone`,
  `Equivalent`, `Partial_Equivalent` ..etc. You can see the description of each
  of these what each `trait` does.

- Finally, a `struct` with no fields is called `unit struct` such as `struct
  LookMaNoFields{};`. This is commonly used when you want to group some
  functionality together even if there's no data.
- There is also `Generic struct` that we will learn to use later in the topic of `Generic`.

```rust
struct LookMaNofields{};
struct Pair<T>{x: T, y: T, };
struct Color(u8, u8, u8);
```

-------

## Example Using Rust Display Trait

In the following example, you will notice that we use the trait `Display` which
will allow us to create a `println!("{})` without a need to specify the
necessity of using `println!("{:#?}")` in Debug Mode. Also you don't need to
include the `#[derive(Debug)]` on top of our implemented struct.

```rust

use rand::Rng;

pub fn rust_structs_traits_and_implementation_fn() {
    let mut point_1: Point = Point::new(0.00, 0.00);
    let mut rng = rand::thread_rng();
    point_generator(&mut point_1, &mut rng);
}

pub struct Point {
    x: f32,
    y: f32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " -> point location : <{:.2},{:.2}>", self.x, self.y)
    }
}

impl Point {
    fn new(x_coord: f32, y_coord: f32) -> Self {
        Self {
            x: x_coord,
            y: y_coord,
        }
    }
}

#[allow(unused_assignments)]
fn point_generator(point: &mut Point, rng: &mut rand::rngs::ThreadRng) {
    for i in 0..100 {
        let x_new: f32 = rng.gen();
        let y_new: f32 = rng.gen();
        point.x = x_new;
        point.y = y_new;
        println!("{}", point);
    }
}
```

## Example of using trait Debug Manually

Instead using `#[dervie(Debug)]`, we can implement this trait Manually using.
In this example, we're manually implementing the Debug trait for MyStruct using
the impl keyword. Inside the implementation, we're using the debug_struct
method of the Formatter to create a debug representation of the struct. We then
use the field method to add each field to the debug representation, and finally
call finish to complete the representation.

- This implementation allows us to print a debug representation of MyStruct
  using the `{:?}` format specifier in println!.

```rust
use std::fmt;

struct MyStruct {
    foo: i32,
    bar: String,
}

impl fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyStruct")
            .field("foo", &self.foo)
            .field("bar", &self.bar)
            .finish()
    }
}

fn main() {
    let my_struct = MyStruct {
        foo: 42,
        bar: String::from("hello"),
    };
    println!("{:?}", my_struct);
}
```

## Implementation of Clone and Copy
- Its sometime important to use the `trait` `Copy` and `Clone` to get remove
  the ownership problem. For example the following `struct` namely `Person` as
  we didn't include the `#[derive(Copy, Clone)]` on top. We will see the code
  as we will implement it manually. The Copy trait is a marker trait in Rust
  that indicates that a type can be copied by simply copying its bytes to
  another location in memory. To implement the Copy trait manually, we need to
  define a clone method that returns a copy of the object. Here's an example
  implementation of the Copy trait for a simple Person struct:


```rust
struct Person {
    name: String,
    age: u32,
}

impl Clone for Person {
    fn clone(&self) -> Person {
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

impl Copy for Person {}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // `person2` is a copy of `person1` because `Person` implements the `Copy` trait
    let person2 = person1;

    println!("Person 1: {:?}", person1);
    println!("Person 2: {:?}", person2);
}

```

- In the example above, we implemented the Clone trait for the Person struct to
  provide a way to make a copy of the object. Then we implemented the Copy
  trait manually for the Person struct by simply adding an empty implementation
  block. This tells the Rust compiler that the Person struct can be safely
  copied by simply copying its bytes.


## Trait Symmary

| idx | Trait Type         | implementation                        | Note                             |
| --- | ------------------ | ------------------------------------- | -------------------------------- |
| 1   | Display            | println!("{}")                        | Use directly to print the object |
| 2   | Debug              | println!("{:?}") or println!("{:#?}") | Use directly to print the object |
| 3   | Iter               | Create iterator                       |                                  |
| 4   | Equavilent         | Comparing two object                  |                                  |
| 5   | Partial_Equivalent | same                                  |                                  |
| 6   | Copy               | Avoid ownership issue                 |                                  |
| 7   | Clone              | same                                  |                                  |

- Any many others.
