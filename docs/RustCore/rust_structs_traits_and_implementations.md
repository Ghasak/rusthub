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

-- Now, you notice that we need to specify `pub` for the struct

```rust

// we have a file named random_info_file.rs as
pub struct RandomInfo{
        pub some_bool: bool, //<- only this will be public
        some_int : i64,      //<- will not be visible
    }
// within same directory of the file above we will have our main.rs


| note |  |
|------|--|


mod random_info_file;
//
use random_info_file::*;
```
