# Computational Tasks

The following is for some computational taks that I need to deal with and to
know more alternative for solving a given problem in `Rust`.

```rust
use std::io;
/// # Demonstration Function
/// ## Function Highlights
/// ### Input
/// The function require some inputs for demonstration.
pub fn life_time_concept_fn() {
    let output = examine_function(10.12, 121.21, 121.23);
    println!("{output:#?}");

    let input_vec: Vec<f64> = vec![121.32, 121.23, 22232.233, 121.44];
    let output = examine_function_2(&input_vec);
    println!("{output:#?}");

    let output: [f64; 4] = examine_function_3(&input_vec);
    println!("{output:#?}")


}
///Simple program show that we can create variable inside a function but must use ownership rule to return it
fn examine_function(param_a: f64, param_b: f64, param_c: f64) -> Vec<f64> {
    //let mut my_vec: Vec<f64> = Vec::new();
    let my_vec: Vec<f64> = vec![param_a, param_b, param_c];
    // my_vec.push(param_a);
    // my_vec.push(param_b);
    // my_vec.push(param_c);
    my_vec
}
///Simple program shows how to deal with creating a vector on Heap
fn examine_function_2(param_ve: &Vec<f64>) -> Vec<f64> {
    let mut output_vec: Vec<f64> = Vec::new();
    for item in param_ve {
        output_vec.push(*item);
    }
    output_vec
}

/// following function allocated on stack for the vector.
/// Vector cannot be change its size (adding more elements or delete elements)
///once its allocated on stack, but you can change the existed values stored already in this vector on stack
//fn examine_function_3(param_ve: &Vec<f64>) -> [f64; 4] {
fn examine_function_3(param_ve: &Vec<f64>) -> [f64; 4] {
    let mut output_stack_vec: [f64; 4] = [0.0; 4];
    for (idx, item) in param_ve.iter().take(4).enumerate() {
        output_stack_vec[idx] = *item
    }
    output_stack_vec
}
```

---

### Example, Creative with &str - related to lifetime

- Take two &str and return which one is larger if parsing possible

```rust

fn main(){
    let s1: &str = "10";
    let s2: &str = "20";
    let output = get_str_re(s1, s2);
    println!("{output:#?}")
    }

fn get_str_re<'a>(param_1: &'a str, param_2: &'a str) -> Option<&'a str> {
    match (param_1.parse::<i32>(), param_2.parse::<i32>()) {
        (Ok(param_1_int), Ok(param_2_int)) => Some(if param_1_int > param_2_int {
            param_1
        } else {
            param_2
        }),
        _ => None,
    }
}
```

### Rule - About static

- You cannot use static MY_VECT: Vec<f63> = vec![1.2, 32.34]; because a
  Vec<f64> is a dynamically-sized type, and its size is not known at
  compile-time. The static keyword can only be used to declare static variables
  with a fixed size and a 'static lifetime.

  ```rust
    static MY_VECT: &[f64] = &[1.2, 32.34];
  ```

- But, later I got, previously stated that this was not possible because
  Vec<f64> is a dynamically-sized type, and therefore not suitable for use with
  the static keyword. However, this statement was incorrect, as Rust does allow
  the definition of static variables with dynamically-sized types under certain
  conditions. Specifically, a Vec<T> can be used as the type of a static
  variable if it is initialized with a constant expression, as is the case in
  the example above.

  ```rust
    // -------- THIS WILL NOT COMPILE --------
    fn test_3<'a>(param_1: &'static Vec<f64>) -> &'static Vec<f64> {
        param_1
    }

    fn main() {
        static MY_VECT: Vec<f64> = vec![1.2, 32.34];
        let my_ref = test_3(&MY_VECT);
        println!("{:?}", my_ref);
    }

  ```

- After, digging inside, we found, the first arguemnt is correct,

  ```rust
  fn main(){

    static MY_VECT: &[f64] = &[1.2, 32.344];
    test_3(&MY_VECT);
      }

    fn test_3(param_1: &'static &[f64]) -> &'static [f64] {
        param_1
    }
  ```

