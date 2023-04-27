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

-----

| idx | note                                                                                            | example                                   |
| --- | ----------------------------------------------------------------------------------------------- | ----------------------------------------- |
| 1.  | It seems `new` function name is the only method for impl for a struct, doesn't need `&str`      | fn new(param_a) almost like a constructor |
| 2.  | You must provide `&self` for every function for any implmented method for the struct,           | fn some_method(&self, param_a)            |
| 3.  | You need `Self` to refere to the `struct` and it is considered as a `type` accessed with (`::`) | let obj = RandomInfo::new()               |
| 4.  | You will need `&self` for accessing data in the constructed `struct` access with (`.`) notation | obj.data_value                            |

-----

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


