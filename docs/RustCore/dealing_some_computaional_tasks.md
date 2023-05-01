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

## Ownership, Reference, lifetime in Struct

Here is an example of a struct with two types of implementations. One has a
field which is consumed by the method in the sturct and later we adopted the
reference instead. This example will deepen our understanding and thinking more
aobut the borrower-checker which causes headache to so many developers.

- Implemntation with moving ownership `no reference` for the heaped allocated `heap_vect`.

```rust

pub fn rust_structs_traits_and_implementation_fn() {
    #[derive(Debug)]
    pub struct RandomInfo {
        pub some_bool: bool,
        pub some_int: i32,
        pub heap_vect: Vec<i32>,
    }
    impl RandomInfo {
        pub fn new(param_a: bool, param_b: i32, param_c:Vec<i32>) -> Self {
            Self {
                some_bool: !param_a,
                some_int: param_b,
                heap_vect: param_c,
            }
        }
        pub fn __str__(&self) -> String {
            let object_def = format!(
                "Current object has: some_bool: {} and some_int: {}",
                &self.some_bool, &self.some_int
            );
            object_def
        }

        /// ## Modifying Function
        /// Note: This function will consume the input as
        /// it will give the ownership to the `some_bool` and
        /// some_int and you cannot use the `param_a` and `param_b`
        /// even though you passed them as reference, because you dereference
        /// them later. But, since both are `stack` not `heap` allocated,
        /// It means they both have trait clone and copy automatically,
        /// as they both cheapkdata.
        pub fn modify_field(&self, param_a: &bool, param_b: &i32, param_c: Vec<i32>) -> Self {
            Self {
                some_bool: *param_a,
                some_int: *param_b,
                heap_vect: param_c,  //<- Already taken the ownership of this variable
            }
        }
    }

    let my_vec = vec![1, 2, 3];

    let mut my_obj = RandomInfo::new(true, 12, my_vec.clone());
    println!("{my_obj:#?}");
    let output = my_obj.__str__();
    println!("{output:#?}");

    let param_a: bool = false;
    let param_b: i32 = 100;

    my_obj = my_obj.modify_field(&param_a, &param_b, my_vec.clone());
    let output = my_obj.__str__();
    println!("{output:#?}");

    let k = 10;
    let c = my_vec; // You can see that we still can use it again.


}
```

- Here, we will find another solutionn by allowing `reference` with `lifetime`
  for the `heap_vect`.

```rust

pub fn rust_structs_traits_and_implementation_fn() {
    #[derive(Debug)]
    pub struct RandomInfo<'a> {
        pub some_bool: bool,
        pub some_int: i32,
        pub heap_vect: &'a Vec<i32>,
    }
    impl<'a> RandomInfo<'a> {
        pub fn new(param_a: bool, param_b: i32, param_c: &'a Vec<i32>) -> Self {
            Self {
                some_bool: !param_a,
                some_int: param_b,
                heap_vect: param_c,
            }
        }
        pub fn __str__(&self) -> String {
            let object_def = format!(
                "Current object has: some_bool: {} and some_int: {}",
                &self.some_bool, &self.some_int
            );
            object_def
        }

        /// ## Modifying Function
        /// Note: This function will consume the input as
        /// it will give the ownership to the `some_bool` and
        /// some_int and you cannot use the `param_a` and `param_b`
        /// even though you passed them as reference, because you dereference
        /// them later. But, since both are `stack` not `heap` allocated,
        /// It means they both have trait clone and copy automatically,
        /// as they both cheapkdata.
        pub fn modify_field(&self, param_a: &bool, param_b: &i32, param_c: &'a Vec<i32>) -> Self {
            Self {
                some_bool: *param_a,
                some_int: *param_b,
                heap_vect: param_c,
            }
        }
    }

    let my_vec = vec![1, 2, 3];

    let mut my_obj = RandomInfo::new(true, 12, &my_vec);
    println!("{my_obj:#?}");
    let output = my_obj.__str__();
    println!("{output:#?}");

    let param_a: bool = false;
    let param_b: i32 = 100;

    my_obj = my_obj.modify_field(&param_a, &param_b, &my_vec);
    let output = my_obj.__str__();
    println!("{output:#?}");

    let k = 10;
    let c = my_vec; // You can see that we still can use it again.
}

```

## Async vs Sync for Factorial Calaculation
- `n` should not be larger than 20 as we will get to overflow.

```rust

use std::time::Instant;
//use tokio::runtime::Runtime;
use std::thread;

pub fn comparison_factorial_results(){

use std::thread;

// Compute factorial of N using sync
fn factorial_sync(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
         n * factorial_sync(n - 1)
    }
}

// Compute factorial of N using async
async fn factorial_async(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let handle = thread::spawn(move|| factorial_sync(n - 1));
        let sub_factorial = handle.join().unwrap();
         n * sub_factorial
    }
}

// Compute factorial of N using async and multi-threading
async fn factorial_async_multithreaded(n: u64) -> u64 {
    if n == 0 {
       1
    } else {
        let num_threads = num_cpus::get();
        let chunk_size = n / num_threads as u64;
        let mut handles = Vec::new();
        for i in 0..num_threads {
            let start = i as u64 * chunk_size + 1;
            let end = if i == num_threads - 1 { n } else { (i as u64 + 1) * chunk_size };
            let handle = thread::spawn(move || {
                let mut sub_factorial = 1;
                for j in start..=end {
                    sub_factorial *= j;
                }
                sub_factorial
            });
            handles.push(handle);
        }
        let mut sub_factorials = Vec::new();
        for handle in handles {
            sub_factorials.push(handle.join().unwrap());
        }
        let mut result = 1;
        for sub_factorial in sub_factorials {
            result *= sub_factorial;
        }
        result
    }
}

    let n = 18;
    // let result_sync = factorial_sync(n);
    // println!("Factorial of {} using sync: {}", n, result_sync);

    // let result_async = futures::executor::block_on(factorial_async(n));
    // println!("Factorial of {} using async: {}", n, result_async);
    //
    let result_async_mt = futures::executor::block_on(factorial_async_multithreaded(n));
    println!("Factorial of {} using async and multi-threading: {}", n, result_async_mt);
}

```
