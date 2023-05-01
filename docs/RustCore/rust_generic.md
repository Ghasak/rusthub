# Rust Generic

We will talk now about `Generic` in `Rust` based on lecture notes by `Doug Milford`.

- Assume we have the following code below. We will have two sets of struct. We
  have created an `explicity` struct of type i32.

```rust

struct Point {
        x: i32,
        y: i32,
    }

fn main(){
        let point_1 = Point{
                x: 23,
                y: 100,
            }
        println!("x = {}, y = {}", point_1.x, point_1.y )
    }
```

- What if we want also a `f32` for `x&y`, it become tedious to create another
  struct for different data type. Instead of create explicit field data type,
  we can use generic to accept any datatype in our given strust. `T` it is
  convenision to select this letter for datatype, but not required. Then use
  the `T` anywhere for your data fields.

```rust

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

let point_1 = Point {
    x: 1012.23,
    y: 1121.44,
};

let point_2 = Point {
    x: 12,
    y: 10
};

// Accept float
println!("{point_1:#?}");
// Accept Int
println!("{point_2:#?}")
```

- If we add more generic to our `struct` we will get even more flexibility to
  assign whatever variable we want. If you hover you mouse

```rust

pub fn generic_in_rust_concept() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let point_1 = Point {
        x: 1012.23,
        y: 1121.44,
    };

    let point_2 = Point {
        x: 12,
        y: 10
    };


    let point_2 = Point {
        x: 12.0,
        y: 10
    };

    let point_2 = Point {
        x: 12,
        y: 10.12,
    };
    // Accept float

    println!("{point_1:#?}");
    // Accept Int
    println!("{point_2:#?}")
}
```


- The `T` is a placeholder until the complier saw you using in explicit manner.
  Once it understand your intend for a piece of data, it's off to the races,
  and treat it as an explici type within your code from there on out. You're
  able to create any data you like that fits the generic defintion given you
  comply with some constraints which we will talk about shortly.

- So, what are `Generic` and why you would like to use them. `Generic` is
  available in other languages and certainly aren't unique to `Rust`, it's a
  way of creating abstarct types both for both `struct`, `enum` and also for
  `function, methods`. We're really creating a placeholder types instead of
  explicityly defining `i32`, `String` , `&str` or whatever. It says' I am
  going to write a code that can work for multiple different types but I don't
  want to have to write individual code for each type I want to handle. This
  will add a significant flexibility not only for your current coding needs,
  but also for future use, so basically:
    - Abstract Types (aka placeholder types)
    - Adds flexibility
    - Reduces Code Duplciation.
    - No runtime cost of using `Generic`.


- You can use the `T` for first Generric. And, if we want another Generic
  variable we can add as many as we need, next `U` can be used.

- You can also constrcut your `Generic` with strict defintion to some type for
  example see below. That we can add an explicit defintion to the data we need.

```rust
struct MyInfo<T>{
        x: i32,
        y: T,
        z: T,
        some_char: char,

    }

```


- I only make things generic if you have a need to support multiple types, it's
  too easy to go down the abastrct rabbit hole until nobady knows what the hell
  your code is trying to do, let's cleanup for the next section.


- `Generic` with `Enum`,

```rust
pub fn generic_in_rust_concept() {
    #[derive(Debug)]
    enum RandomInfo<T> {
        OptionA(T),
        OptionB(T),
        OptionC,
    }

    let my_data: RandomInfo<f32> = RandomInfo::OptionA(100.23);
    let my_other_data: RandomInfo<f64> = RandomInfo::OptionB(400.23);
    let last_data: RandomInfo<f32> = RandomInfo::OptionC;


    let some_option:RandomInfo<&str> = RandomInfo::OptionA("This is just  a test ...");

    println!("{my_data:#?}");
    println!("{some_option:#?}");
}```
