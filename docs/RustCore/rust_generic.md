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
