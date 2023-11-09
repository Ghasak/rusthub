# Learning about Module, Public, Use and Crates

## Starting point
Every project has its own root directory which we will use to trigger our
complier. It means the `starting-point` of the given project, is located at the
root-directory. Hence, you can use the `relative path` or the `absolute path`
without any problem.

## Content

Private and brinnig modules into scope using the used keyword. So modules are a
way fo rus to orgnaize our rust project, they work similary to java or
javascript modules, it gives us better readability and better code
reuseability. To show all the different ways we can use modules, we will be
going over modules

### A. with the single file first (inside the main.rs file)

Inside our `mod` we can have
- variables
- functions
- or other modulus


**NOTES**
- By default, all functions inside `mods` are `priavte` and we need `pub`
  keyword to make our module public. That inclues also the constant, variables
  `mods`.

- When you declare a mod in a single file within your `main` file, it is not
  necessary to use `pub:public` as it is alreayd within the given file, and
  considered private by default, but can be seen by the the file itself.


#### Using Modules

1. Create a module called `house`, that has two other modules called `bedroom1`
and `bedroom2`.

```rust
mod house {

    pub const HOUSE_NUMBER: u32 = 56;
    pub mod bedroom1 {
        pub fn is_light_on() -> bool {
            println!("We have selected the function : bedroom1");
            false
        }
    }

    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            println!("We have selected the function : bedroom2");
            true
        }
    }
}


```


2. Now, we can call them, in our `main` function using:

```rust


fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        house::HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        house::bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        house::bedroom2::is_light_on()
    );
}

```
3. `use` keyword, allow us to bring our `mod` in our current scope, then, it
   will not be needed to reference in the `println!` as `house::bedroom1`,
   instead, you can immediately use the function within this `mod`
   `is_light_on()`

```rust
use house::bedroom1

fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        house::HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        house::bedroom2::is_light_on()
    );
}

```

- You can also write multiple `use` with curly-bracket using
```rust

use house::{bedroom1, bedroom2, HOUSE_NUMBER};

fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        //house::HOUSE_NUMBER
        HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        bedroom2::is_light_on()
    );
}


```

#### Calling modules from antor module

There are three kewrods associated with the `use` keyword these are:
- `crate` which is associated with our current rust  project such as `use carte::house::`
- `self` which is associated with our current module we are in.
- `super` which is assoicated with the parent module we are in.


```rust
// use std::io ;
// use std::io::stdin;
// these above two calles can be summarized into
use std::io::{self, stdin};

```

```rust
mod house {

    pub const HOUSE_NUMBER: u32 = 56;
    pub mod bedroom1 {
        pub fn is_light_on() -> bool {
            println!("We have selected the function : bedroom1");
            false
        }
        pub fn is_neibours_bedroom_light_on() -> bool {
            use super::bedroom2;         // <- this is the usage of super, as we can access the house moudle (parent module)
            println!(
                "checkign our neighbor light is on? {}",
                bedroom2::is_light_on()
            );
            bedroom2::is_light_on()
        }
    }

    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            println!("We have selected the function : bedroom2");
            true
        }
    }
}


use house::{bedroom1, bedroom2, HOUSE_NUMBER};

fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        //house::HOUSE_NUMBER
        HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        bedroom2::is_light_on()
    );

    println!(
        "Checking the super usage ... {}",
        bedroom1::is_neibours_bedroom_light_on()
    );
}


```

### B. Inside many files (modules inside multiple files )
Now, we want to move the `house` module to a speparated file, we need first
- Create a file within the `main.rs` directory, called `house.rs` which is
  similar name to our `house` <=> `parent module`

```shell
├─ INSERT  19304d5h34m|master ?4
╰─ lsd
  rw-r--r--   1   gmbp   staff      0 B     Tue Nov  8 14:34:27 2022    house.rs
  rw-r--r--   1   gmbp   staff      1 KiB   Tue Nov  8 14:29:22 2022    main.rs
```


- Inside `house.rs`, remove the parent module and child parent module `mod` keyword, since they are now living on sepearted file.

```rust
pub const HOUSE_NUMBER: u32 = 56;
pub mod bedroom1 {
    pub fn is_light_on() -> bool {
        println!("We have selected the function : bedroom1");
        false
    }
    pub fn is_neibours_bedroom_light_on() -> bool {
        use super::bedroom2;
        println!(
            "checkign our neighbor light is on? {}",
            bedroom2::is_light_on()
        );
        bedroom2::is_light_on()
    }
}

pub mod bedroom2 {
    pub fn is_light_on() -> bool {
        println!("We have selected the function : bedroom2");
        true
    }
}

```
- Inside `main.rs` you can call the `house.rs` using `mod house`, and then use
  the `use` keyword to select what you want from this `house` module.

```rust
#![allow(dead_code)]

mod house;
use house::{bedroom1, bedroom2, HOUSE_NUMBER};

fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        //house::HOUSE_NUMBER
        HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        bedroom2::is_light_on()
    );

    println!(
        "Checking the super usage ... {}",
        bedroom1::is_neibours_bedroom_light_on()
    );
}

```

### C. Inside directories ( modules inside multiple directories )

Now, our module becomes too big, and we need to break it down into several
files that lives inside a given directory. Thinking of the structure as
- You must remove the `mod` with their `curly-brackets` at each module to insure, it is called.

- house (`directory`)
    - bedroom1 (`file`)
        - is_light_on(...) (`function in the file`)
    - bedroom2 (file)
        - is_light_on(...) (`function in the file`)

```shell
╭─ gmbp   GMacBookPro on ~/Desktop/devCode/rust_fundamentals/learning_rust/src   
├─ﮧ INSERT  19304d6h0m|master ?4
╰─ ctree .

 └──     house/
 │  └────     bedroom1/
 │  │  └────     mod.rs
 │  └────     bedroom2/
 │  │  └────     mod.rs
 │  └────     mod.rs
 ├──     house.notused
 └──     main.rs
```


- At `bedroom1` directory, we have a `mod.rs`, it has

```rust

pub fn is_light_on() -> bool {
    println!("We have selected the function : bedroom1");
    false
}
pub fn is_neibours_bedroom_light_on() -> bool {
    use super::bedroom2;
    println!(
        "checkign our neighbor light is on? {}",
        bedroom2::is_light_on()
    );
    bedroom2::is_light_on()
}
```
- At `bedroom2` directory, we have a `mod.rs` it has:
```rust

pub fn is_light_on() -> bool {
    println!("We have selected the function : bedroom2");
    true
}
```

- Then, we have at parent directory `house` similary `mod.rs` it has:

```rust
pub const HOUSE_NUMBER: u32 = 56;
pub mod bedroom1;
pub mod bedroom2;

```
- Finally, at our `main.rs` we can now call the module `house` using `mod
  house`, then we can specify what we want to call from it using `use` keyword

```rust

#![allow(dead_code)]

mod house;
use house::{bedroom1, bedroom2, HOUSE_NUMBER};

fn main() {
    println!("Hello, world!");
    println!(
        "Our house status number is          : {}",
        //house::HOUSE_NUMBER
        HOUSE_NUMBER
    );
    println!(
        "Is the light on? in our first house : {}",
        bedroom1::is_light_on()
    );
    println!(
        "Is the light on? on second house    : {}",
        bedroom2::is_light_on()
    );

    println!(
        "Checking the super usage ... {}",
        bedroom1::is_neibours_bedroom_light_on()
    );
}

```

### Example, with various calling type

1. Create `module` called `employee_module` and put inside of it a file called `mod.rs`.

```rust
pub const HOUSE_NUMBER: u32 = 56;


// Direct definition calling
pub fn create_employe(n:u32){
    for i in 1..n{
        println!("We have created: {i}");
    }
}


// You can also call scripts (mods) from within your current directory, to be included.
pub mod some_function;


// You can also call sub-module inside another directory

pub mod apple_module;

```
2. Create a file called `some_function.rs` , as a simple file with

```rust
pub fn print_something(){
    println!("This is just a function inside our module ... ");
}

```

3. Create a directory called `apple_module` with a `mod.rs` inside of it


```rust
pub fn my_apple_fn(){
    println!("This is the apple function called from the apple_module");
}

```

4. Now, you head back to your `Crate root` which is `src/main.rs` and use:

```rust
//use empolyee_module::create_sub_module ;
mod employee_module;
use employee_module::{create_employe, HOUSE_NUMBER, some_function,apple_module};


fn main() {
    println!("{}", HOUSE_NUMBER);
    let n = 10;
    create_employe(n);
    some_function::print_something();
    apple_module::my_apple_fn();
}

```

**Note**:
- If you change the name of `apple_module` directory to be same as the file
  `some_function`, thats where the `some_function.rs` will be called first. and
  it will not look into the `directory`
- If you want to remove `mod.rs` you can use `employee_module.rs` inside the
  `emplyee_module` directory and put all the `mod.rs` calling inside of it.




## NESTED MODEL IN NUTSHELL

**NOTE** At each directory we are making `mod.rs`, we must create a `mod.rs` which is
similar to `init.lua` in `lua` or `__init__.py` in `python`. It will help us to
call the directory as a modular without refering to any sup files inside of it.
That was in `Rust-2015`, as for now, for `Rust-2018`, it is no longer being
required, instead we name `mod.rs` -> `<same_module_name>.rs` (same_module_name
means the directory where we put all our `rust-files` inside). More expliantion is down here:

### RUST 2018

It's no longer required to have the file `mod.rs` (although it is stil
supported). The idiomatic alternative is to name the file the name of your
module:

```rust
    src
    ├── main.rs
    ├── my
    │   ├── inaccessible.rs
    │   └── nested.rs
    └── my.rs
```
In the `main.rs` use

```rust
mod my;
fn main(){
        my::function();
    }
```
In `my.rs`

```rust
pub mod nested; // if you need to inclucde other modules
pub fn function(){
        println!("called `my::function()`")
    }

```


### RUST 2015

You need to put a `mod.rs` file inside your folder of the same name as your
module. `Rust by Example` explains it better:

```shell
    $ tree src
    src
    ├── main.rs
    └── my
        ├── inaccessible.rs
        ├── mod.rs
        └── nested.rs
```
In `main.rs` you can use:
```rust
mod my;

fn main(){
        my::function();
    }
```

In `mod.rs` :
```rust
pub fn function(){
        println!("called `my::function()`");
    }
```

## SIDE NOTES FROM RUST BOOK

- `crate root` is the place where your `rust complier` is start looking for
  files at beginning, that includes `main.rs, src/main.rs, src/lib.rs`.
- `file` first then `directory` after, is what `rust complier` will fetch first
  (especialy if you have file at parent directory has same name as the child
  module (directory name))

- When you create `dir` you have two options (e.g., file not found for module
  `concepts` to create the module `concepts`, create file `src/concepts.rs` or
  `src/concepts/mod.rs`)
    - Put `mod.rs` inside the directory  (Rust 2015)
    - Same level of `dir` put `<same_dir_name>.rs` (Rust 2018)


- `2023-07-04 22:54` In Rust the equavlent concepts `namespaces` in `cpp` is the `mod`. As we
already seen, Modules in Rust are used to group related functions, structs, and
other items together. They also provide a way to contorl the visibility of
items, which is similar to the way namespaces contorl the visibility of
declaration in C++.

## Tools for investigating the project skelton in Rust
You will need a `Crate` called `cargo-modules` which will allow us to visulize the our developed crate modules tree.
```rust
cargo install cargo-modules
```
Associated commands with it:
```rust
cargo modules generate tree
```
- Checking our module tree using:
```rust
╰─ cargo modules generate tree

crate rust_learning_hub
└── mod concepts: pub(crate)
    └── mod ch01: pub
        ├── mod common_collections: pub
        ├── mod experimental_ideas: pub
        └── mod ownership_borrowing: pub
```
To see other features you can use `cargo modules generate tree  --help`
- Now, lets even the `enum` `struct` ..etc using
```rust
├─ﮧ INSERT  1h28m|main !8 ?1
╰─ cargo modules generate tree --with-types

crate rust_learning_hub
├── mod concepts: pub(crate)
│   ├── mod ch01: pub
│   │   ├── mod common_collections: pub
│   │   │   ├── fn common_collections_fn: pub
│   │   │   ├── fn string_to_static_str: pub
│   │   │   └── fn type_of: pub
│   │   ├── mod experimental_ideas: pub
│   │   │   └── fn experiment_sum_fn: pub
│   │   └── mod ownership_borrowing: pub
│   │       ├── fn about_owner_ship_concepts: pub
│   │       ├── fn func1: pub
│   │       └── fn square: pub
│   └── fn create_text: pub
└── fn main: pub(crate)
```


## REFERNCES
- [07 Modules, Pub and Use | Rust Tutorials](https://www.youtube.com/watch?v=6cfcWzsvLrA)
- [Rust Book](https://www.youtube.com/watch?v=6cfcWzsvLrA)
- [How can I include a module from another file from the same project?](https://stackoverflow.com/questions/26388861/how-can-i-include-a-module-from-another-file-from-the-same-project)
- [Rust Modules - Explained Like I'm 5 ](https://www.youtube.com/watch?v=969j0qnJGi8)
