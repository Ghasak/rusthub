# Rust OwnerShip and Borrowing
The owenership and borrowing are why rust is rust.
- Initially rurstrated both new and senior developers.
- Memeory management.
    - No garbage colection
    - Developer manages memeory in code
    - stack vs heap
- benefit
    - Runtime speed
    - Parallel and concurrent processing
    - Safety


## Comparing STACK VS HEAP OWNERSHIP

| idx   | STACK                                                                                                  | HEAP                                                                              |
| ----- | ------------------------------------                                                                   | --------------------------------                                                  |
| 1     | Stack is allocate on stack memeory                                                                     | Heap allocated on HEAP memeory                                                    |
| 2     | Easy to copy                                                                                           | Expensive to copy                                                                 |
| 3     | Ownership is not necessary                                                                             | Ownership is necessary to be maintained                                           |
| 4     | It will create copy when necessary, by default in rust complier, when there is a conflict in ownership | By default you must use reference or clone to aovid the conflict in the ownership |
| 5     | For declare a stack variable: No need to define mutability, by default its already mutable             | For declar Heap variable: Need to define mutiability, by default, you must provide the keyword `mut`        |
| 6     | Stack doesnt required to use (&) borrowing ownership, as its on by  default                            | Need to specify the borrowing (&)                                                 |


## Concept
Assume we have the following example

```rust
let stack_i8: i8 = 10;
let stack_f32: f32 = 20.;
let stack_bool: bool = true;
let stack_char: char = 'a';
```
They will be allocated in the `stack` as:
```
+---------------+
|   stack_char  |
+---------------+
+---------------+
|   stack_bool  |
+---------------+
+---------------+
|   stack_f32   |
+---------------+
+---------------+
|   stack_i8    |
+---------------+
```
## Generally speaking:
- Every piece of data in memeory has an owner (stack or heap)
- Heap allocated memeory is cleaned up automatically when the last 'owner' of
  the memeory goes out of scope.

## STACK
- STACK Defintion
    - Fast memeory creation  and retrieval .. speed , speed , speed!
    - Memory is automatically recaptured by the program after variables go out of scope
    - I sthe default in Rrust
    - Fixed size variables .. Collections Cannot be stack based ( and String are a collection of u8 's')
- Stack then,
    - known, fixed size type in memeory
    - No need to search, they stack on top of each others,
    - very fast
    - Once they cleaned they pop from top with the critera (`LIFO`), Last Inter First OUT.

```rust
// STACK
let stack_i8: i8 = 10;
let stack_f32: f32 = 20.;
let stack_bool: bool = true;
let stack_char: char = 'a';
let stack_vec: [&str; 4] = ["north", "south", "west", "east"];
```

## HEAP
On Heap, we create the variables on the heap
- Flexiablitiy.
- Memeory that can grow in size (Vector, HashMap, String, .. etc. )
- Runtime performance cost (speed)
- Memeory that can live beyond the scope that created it.
- Memory is automatically recaptured when the last OWNER goes out of scope.
```rust
//HEAP
let heap_vector: Vec<i8> = Vec::new();
let heap_string: String = String::from("Hello");
let heap_i8 : Box<i8> = Box::new(10);
let mut heap_bool: Box<bool> = Box::new(true);
```
### Example 1. Assume we create the following

```rust
let heap_i8 : Box<i8> = Box::new(10);
let heap_i8_2 = heap_i8;
println!("{}", heap_i8);
```
#### How it works
1. The `heap_i8` is the owner of the data (10) allocated on the heap
2. We transfered the ownership from `heap_i8` to a new variable called
   `heap_i8_2`.
3. Since you cannot have two owners for same data allocation, rust complier
   will drop the variable `heap_i8` and its got cleaned in memeory.
4. Therefore, the above will produce an error `borrow of moved value: heap_i8
   value borrowed herre after move`.

#### Why?
This is in stark contrast to most other languages. Most languages will simply
have two variables pointing to the same allocated heap memory. And, when they
do they open up a Pandora's box issues which we'll disucss later.but mainly we can say:
- Other langauges: two varaibles can point to the same memeory, making parallel
  adn concurrency issues such as `race condition` hard to be tackled.
- **Notice** that we have this issue only on `HEAP` not on `STACk`, because on
  Stack, creating a copy is very cheap, so Rust complier will create a copy
  automatically.

#### How you can solve this?
There are several ways to resolve the borrowing and moving issue above.

1. Don't use this variable `heap_i8`.
2. Borrow the ownership
- using the (`&`) to borrow the ownership from the variable, and retrun the
  ownership to the original variable once the variable that borrowed the
  ownership goes oout of the scope.
3. Create a clone:
- This will create a new heap allocated variable, but, this  will be tedious as
  we need to clone every variable to avoide the ownership.
- `Cloning` is relatively expensive on the heap, and using reference instead is
  our best alternative.
- We will write:
```rust
let heap_i8 : Box<i8> = Box::new(10);
//-- > let heap_i8_2 = heap_i8;
let heap_i8_2 = heap_i8.clone();
println!("{}", heap_i8);
```

### Example 2. Borrowing with Procedure
#### ON STACK
Borrowing with procdure (method or functions), is same, the ownership will be
passed from the original variables to the params of the procedure signture (the
procedure parameters). This point was not clear before for me.

**Reminder**:
- A parameter is the variable listed inside the parentheses in the function
  definition. An argument is the value that is sent to the function when it is
  called., so in the following example argument is the `stack_f64`.

##### How it works
1. We created a stack argument with value of `1.0`.
2. Then, we created the `stack_procedure` to perform some computational task.
3. The stack procedure parameter namely `param`, will take the ownership of the stack,
4. Since we know `stack` allocated memeory doesn't require to concern about
   ownership, we don't need to do anything, but to understand what the complier
   does, is that it will create an exact copy of the `stack_i8` in background,
   and pass it to the param in the `stack_procedure`.
5. You can see that we still be able to access the original variable `stack_i8`
   even after we passed its ownership to the `param` as Rust created a copy as
   we said, without our need to concern about this.
6. We can see the the original variable `stack_i8` is intact.

```rust
// STACK
fn main(){
    let stack_f64: f64 = 1.0;
    stack_procedure(stack_f64);
    println!("Original value on stack = {stack_f64}");
    }

fn stack_procedure(mut param: f64) {
    param +=1.0;
    println!("In stack_procedure with param: {param}");
}
// Result
//In stack_procedure with param: 2
//Original value on stack = 1
```
#### ON HEAP
Now, we will run into an issue of the `borrowing`, As we mentioned before, the
ownership is related to heap allocation not stack allocation.

##### How it works
1. We did exactly similar to the example above. The Ownership of memeory
   associated with `heap_f64`a gets transferred to `param`.
2. Notice,that the param is now the owner, and we have passed the argument
   `heap_i8` which is now no logner available, (there will be only one owner at
   a time).


```rust
fn main(){
    // HEAP
    let heap_f64: Box<f64> = Box::new(1.0);
    heap_procdure(heap_f64);
    println!("Original Heap value on stack = {heap_f64}"); // <- You will get a complier error that ownership moved to param.
    }
fn heap_procdure(param: Box(f64))
{
    println!("{param}",);
}
```
##### Solutions
1. Now, we can use either `clone`.
But, this is so expensive, imagine you have a vector of 1 million records, and clone this vector is inefficient.

```rust

fn main(){
    // HEAP
    let heap_f64: Box<f64> = Box::new(1.0);
    heap_procdure(heap_f64.clone());
    println!("Original Heap value on stack = {heap_f64}"); // <- You will get a complier error that ownership moved to param.
    }
fn heap_procdure(param: Box(f64))
{
    println!("{param}",);
}
```
2. Pass back the ownership from the procedure to the original variable. (Also, its shadowing)
- This works but
   tedious as we need to manage the return,  what if we have several
   parameters? basically the solution here is just to make us understand the
   ownership.
    - Notice, we pass the ownership from the `heap_f64` to the param at first
      (signture of the heap_procdure).
    - Then, we allow to return same parameter in our `heap_procedure` but including
      the variable at the end without a `;`.
    - Then, when we call the procedure, we pass it back to the argument `heap_f64`.

```rust

fn main(){
    let mut heap_f64: Box<f64> = Box::new(1.0);
    heap_f64 = heap_procdure(heap_f64);
    println!("Original Heap value on stack = {heap_f64}");
    }

fn heap_procdure(param: Box<f64>) -> Box<f64> {
    println!("{param}",);
    param
}

```
- The benefit, it is not using cloning, but hard and not easily to be
  maintained, when you have so many param in the signture, see below

```rust
fn main(){
    // HEAP
    let mut heap_f64: Box<f64> = Box::new(1.0);
    let mut heap_bool: Box<bool> = Box::new(true);
    (heap_f64,heap_bool) = heap_procdure(heap_f64, heap_bool);
    println!("Original Heap value on stack = {heap_f64}, with bool: {heap_bool}");
    }

fn heap_procdure(param: Box<f64>, param_b: Box<bool>) -> (Box<f64> ,Box<bool>){
    println!("{param}",);
    (param, param_b)
}

```

3. [Most efficent way] Using reference `&`
- We can use the reference,
```rust
fn main(){
    // HEAP
    let heap_f64: Box<f64> = Box::new(1.0);
    heap_procdure(&heap_f64);
    println!("Original Heap value on stack = {heap_f64}");
    }
//fn heap_procdure<T: std::fmt::Display>( param: &Box<f64>){
//fn heap_procdure<T: std::fmt::Display>( param: &Box<T>){
//fn heap_procdure<T: std::fmt::Display>( param: &T){
fn heap_procdure(param: &Box<f64>) {
    println!("{param}");
}
// Or use this efficent
fn heap_procdure<T>(param: &T)
where
    T: std::fmt::Display,
{
    println!("{param}",);
}
```

### What is Borrow? and Move?
- `Borrow` means using reference, `&` for any param when is created, it can
  borrow ownership from original argument, then return it after it goes out of
  the scope to the original argument.
- `Move` means, the ownership is moved from one variable to another, such as
  from our heap_f64 argument to the param in the param_procedure. Once, its
  moved, the original argument will be cleared from memeory as its not allowed
  to have several owners to same piece of data.


## String and String Slices
Similarly, we have also to think of the `String` as it is allocated on Heap,
### String Slices
- String slicies is a pointer .. to either stack or heap. It is not allocated
  on `stack` or `heap` but it borrow the ownership of a memeory allocated of
  someone elses. Thats why you see `&` in the slice.
- `some_str`, is not affected by the ownership issue, as its allocated on stack.
- In the example down below, the ownership is tarnsfered from `some_string`
  `allocated on heap` to the parameter `string_heap` in the `some_procedure`.
  Thus, we need to borrow the ownership of the `some_string` and given it back
  once the `string_heap` reach to end of procedure and cleaned from memeory.

```rust
pub fn rust_owner_ship_and_borrowing_concept() {
    // String on Heap
    let some_string: String = String::from("Partner");
    // String Slices
    let some_str: &str = "Hello";
    some_procedure(some_str,&some_string);
    println!("{some_str}");
    println!("{some_string}");

}

fn some_procedure(string_slice: &str, string_heap: &String){
    println!("{string_slice} {string_heap}");
}

```


## Ownership with mutiability
As we stated, we can for heap data have only one owner at a time for same data.
We can have multiple references at a time, but with two conditions.
- The original argument must be immutable (doesnt change).
- True owner in the example below is always `var_a` while `var_b` and `var_c` are both just references.
```rust
let var_a = String::from("Howday!");
let var_b = &var_a;
let var_c = &var_a;
println!("{var_a} {var_b} {var_c}");
```
- The Rust complier assumes that the `var_a`, will not change down stream,
  hence, it doesn't care that assign many references for different variables
  like `var_b` and `var_c`.
- This assumption if `var_a` DOES change, Rust will produce a compile error.
  You can see below, that we violated this assumption, as we used the println!
  statement while the data allcaated on heap `var_a` is already altered.

```rust
let mut var_a = String::from("Howday!");
let var_b = &var_a;
let var_c = &var_a;
var_a.push('a');
println!("{var_a} {var_b} {var_c}");
```
- but, if you use the references before we alter, it passes, and no complier
  error as shown below, as the jop is ended prior to the mutation.

```rust
let mut var_a = String::from("Howday!");
let var_b = &var_a;
let var_c = &var_a;
println!("{var_a} {var_b} {var_c}");
var_a.push('a');
```
- Or if you also altered the heap data before creating the references. it passes.
```rust
let mut var_a = String::from("Howday!");
var_a.push('a');
let var_b = &var_a;
let var_c = &var_a;
println!("{var_a} {var_b} {var_c}");
```
- In summary, either, you manage the mutiability, while creating the references, or use `clone` when it is needed.

```rust

pub fn rust_owner_ship_and_borrowing_concept() {
    let var_a : String = String::from("Howday");
    let var_b : String = String::from("Partner");
    let var_c : String = String::from("Cowabanga!");
    let mass_data : Vec<&str> = vec![&var_a, &var_b, &var_c];  //Could be millions of records
    let k: f64 = heavy_calc(mass_data);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn heavy_calc(param: Vec<&str>) -> f64{
    // Some heavy duty clacs performed here that utilize available cores of my computer
    for param in param{
        println!("{}", param)
    }
    32.23

}

```

## Ownership in Struct

- We can ask the question, why copy and clone, is not assume as default for the
  struct, instead, we need to implement by ourselves.
  - Possible answer, the potential strcut can have thousands of fields, and assume copy by default is for performance issue.
  - It is more logical to assume the creator of Rust, enforce the developer to implement the clone or copy by themselves.
- Similary, we use the `&` to allow the struct to be referenced.
```rust

#[derive(Debug)]
struct my_struct {
    a: i32,
    b: f64,
}

pub fn rust_owner_ship_and_borrowing_concept() {
    let var_1 = my_struct { a: 9, b: 10.0 };
    some_procedure(&var_1);
    println!("{:#?}", var_1);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn some_procedure(param_a: &my_struct) {
    println!("{:?}", param_a)
}
```
- If you still, want to implement the `copy`/`clone` you can use, then you
  don't need reference anymore. Notice, that the `tarit` `copy` is the one that
  replace the necessaty of using .clone() method when using the strct in the
  procedure, as it will creat a copy for us automatically.

```rust

#[derive(Debug, Clone, Copy)]
struct my_struct {
    a: i32,
    b: f64,
}

pub fn rust_owner_ship_and_borrowing_concept() {
    let var_1 = my_struct { a: 9, b: 10.0 };
    some_procedure(var_1);
    println!("{:#?}", var_1);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn some_procedure(param_a: my_struct) {
    println!("{:?}", param_a)
}
```
- You can also use the `clone` implementation manually as following
```rust
#[derive(Debug)]
struct my_struct {
    a: i32,
    b: f64,
}
impl Clone for my_struct{
    fn clone(&self) -> Self{
        Self{a: self.a, b: self.b}
    }
}

pub fn rust_owner_ship_and_borrowing_concept() {
    let var_1 = my_struct { a: 9, b: 10.0 };
    some_procedure(var_1.clone());
    println!("{:#?}", var_1);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn some_procedure(param_a: my_struct) {
    println!("{:?}", param_a)
}
```
- Not all strcuts are able to have copy trait, such as

```rust
#[derive(Debug, Clone)]
struct my_struct {
    a: i32,
    b: f64,
    c: String,  // <- we have added this part only
}

pub fn rust_owner_ship_and_borrowing_concept() {
    let var_1 = my_struct { a: 9, b: 10.0 , c: String::from("Wow")};
    some_procedure(var_1.clone());
    println!("{:#?}", var_1);
}

#[allow(unused_variables)]
#[allow(unknown_lints)]
#[allow(renamed_and_removed_lints)]
#[allow(uninlined_format_args)]
fn some_procedure(param_a: my_struct) {
    println!("{:?}", param_a)
}

```
- Finally, mutiability with strcut while considering ownership

```rust

#[derive(Debug)]
struct my_struct {
    a: i32,
    b: f64,
    c: String,
}

pub fn rust_owner_ship_and_borrowing_concept() {
    let mut var_1 = my_struct { a: 9, b: 10.0 , c: "Wow".to_string()};
    some_procedure(&mut var_1);
    println!("{:#?}", var_1);
}

fn some_procedure(param_a: &mut my_struct) {
    println!("{:?}", param_a)
}
```
## RUST OWNERSHIP AND BORROWING
- Why is this necessary?
    - Eliminates memeory issues (null pointers, dangling pointers, data races ..etc.)
    - Eliminates the Garbagge Collector.
    - Parallel processing is a breeze!
- Mastery of Ownership and Borrowing will take time
    - Be patient :).
    - Stick with it.

### Example - Ownership is not an issue for stack variables

- In this code, you are not using any reference or borrowing. The some_int_var
  is a simple integer variable, and when you pass it to the get_int_ref
  function, the value is moved into the function and is no longer accessible in
  the calling function. The get_int_ref function simply takes an i32 value as
  input and returns it as output. Therefore, there is no ownership issue in
  this code.
- In the code you provided, some_int_var is not being borrowed by any
  references or passed as an argument to the get_int_ref function. Therefore,
  it is not being moved or borrowed, and the ownership remains with the scope
  in which it was declared, which in this case is the function
  life_time_concept_fn.
- When you call the get_int_ref function with some_int_var as an argument, Rust
  makes a copy of some_int_var and passes it to the function. The get_int_ref
  function returns this copy, and the value of some_int_var is not affected.
- The returned value is then assigned to new_varome_int_var, which becomes the
  new owner of the value. When println!("{new_varome_int_var}"); is executed,
  new_varome_int_var is printed. Finally, println!("{some_int_var}"); is
  executed, and the value of some_int_var is printed without any issues since
  it was not moved or borrowed.

```rust
pub fn life_time_concept_fn() {
    let some_int_var : i32 = 10;
    let new_varome_int_var = get_int_ref(some_int_var);
    println!("{new_varome_int_var}");
    println!("{some_int_var}");

}

fn get_int_ref(param_1: i32 )-> i32{
        param_1
}
```
- But, the code should not be compiled and I verified this with `Rust: v.1.5.8.0`. As we get an erro messge says
```rust
error[E0382]: use of moved value: `some_int_var`
 --> src/main.rs:5:26
  |
3 |     let some_int_var: i32 = 10;
  |         --------------- move occurs because `some_int_var` has type `i32`, which does not implement the `Copy` trait
4 |     let new_var = get_int_var(some_int_var);
  |                                   ---------- value moved here
5 |     println!("{some_int_var}"); // This line causes a compilation error
  |                          ^^^^^^^^^ value used here after move
  |
  = note: move occurs because `some_int_var` has type `i32`, which does not implement the `Copy` trait

```

- But, later, as I start using the complier `Rust: rustc 1.67.0 (fc594f156
  2023-01-24)`, I start being able to complie, and the reason is:
    - It seems that starting from Rust 1.45, the Copy trait is automatically
      implemented for types that implement Clone, and i32 is one of these types.
      Therefore, the code you provided earlier will compile without any issues on
      Rust 1.67.0.
- **Summary**,
    - For stack variables, they are cheap, so the `clone/copy` is implemented
      automatically. you will face this issue once you use the `heap` allocated
      data types.

