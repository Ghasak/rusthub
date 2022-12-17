# Understanding more ownership, borrowing and references

## A. On Stack
Anyting on stack, doesnt require us to dereference it as we know its size at
compliation. The `capacity` will not grow in `run-time` hence we don't need to
care about `ownership`.

For example, the following program, can run for both by refererencing or
dereferenceing the given variable.
- So you can remove the `&width` or `&hight` from both at decleration and
  inside the function signture.

```Rust
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
//#[derive(Debug)]

fn main(){
    let width: u32 = 30;
    let hieght:u32 = 50;

    println!(
    "The area of the rectangle is {:?} square pixels",
        area(&width, &hieght)
)

}


fn area(width: &u32, hight: &u32) -> u32{
    width * hight
}

```
This includes,

- Primitive types (like i32, u32 ..etc.)
- List literal like `let mylist: [&str, 3] = ["element_1", "element_2", "element_3"]`
- List primative like `let mylist: [i32, 3] = [23,34,12]`
- Tuple, with primitve same type or string literal
- Anyting on stack


#### Another example
Here, we passing a `tuple` which has primitive types.
```rust
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
//#[derive(Debug)]

fn main(){

    let rect1: (u32, u32) = (30, 50);
    println!(
    "The area of the rectangle is {:?} square pixels",
        area(rect1)
)

}


fn area(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

```




## Important Note
What we have stated above, is correct for `primative types` which is the one we
are having such as `u32` or `u64` ..etc. So, even if we move (`ownership`) from
one usage of a function to anther same, the compilar will not complain.

```Rust
fn main(){
    let width: u32 = 30;
    let hieght:u32 = 50;

    println!(
    "The area of the rectangle is {:?} square pixels",
        area(width, hieght)
);

    println!(
    "The area of the rectangle is {:?} square pixels",
        area(width, hieght)
)
}


fn area(width: u32, hight: u32) -> u32{
    width * hight
}

```
In this case the output


```shell
╰─ cargo run --quiet
The area of the rectangle is 1500 square pixels
The area of the rectangle is 1500 square pixels

```
but, if we apply same thing for a `struct` which is allocated on the `heap` by
default, then we must care about the `ownership` and must use `references`,
check here:

```rust
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#[derive(Debug)]

struct Dimensions {
    width: u32,
    hight: u32,
}

fn main() {
    let my_dimensions = Dimensions {
        width: 30,
        hight: 50,
    };

    println!("my current dimension_struct is : {:?}", my_dimensions);
    // Since the struct contains primiatives, we don't care about ownership,
    println!("My area is = {}", area(&my_dimensions));

    //If we care about ownership, lets pass a reference, which the compiler will not object
    println!(
        "My area is = {}",
        area_with_reference_ownership(&my_dimensions)
    );
}

fn area(dimension_struct: &Dimensions) -> u32 {
    dimension_struct.width * dimension_struct.hight
}
fn area_with_reference_ownership(dimension_struct: &Dimensions) -> u32 {
    dimension_struct.width * dimension_struct.hight
}

```
```shell
╰─ cargo run --quiet
my current dimension_struct is : Dimensions { width: 30, hight: 50 }
My area is = 1500
My area is = 1500
```

The above code will work only if we use the `&` (to borrow the owndership) of
the struct `Dimensions`.

### freeing memeory
You dont need to worry about freeing the memeory, as it will be freed
automatically once the scope is over.


## Ownership and borrowing concept

1. No Garbage Collector (GC):
- C, C++
    - Very fast and performant.
    - but, requires more work from developers.
- python, Java, Go, JavaScript
    - sacrificies some speed.
    - allow developers to focus on application logic.

- Rust Ownership System
    - Best of both worlds
    - Complier replaes most of GC's responsibilities.
        - Determines at complie time when memeory is allocated and deallocated.
        - Requires developers to code in specific way.

### Rules Ownership Rules
1. each value in rust hass a vairable that's called its owner.
2. there can bonly be one owner at a time.
3. when the owner goes out of socope., the value wil be dropped.


Assume you have now the following code
```Rust
fn main(){
    let s1: String = String::from("Hello World! ");
    let s2 = s1;
    println!("{}", s1);
    println!("{}", s2);

    }
```

- When we declared and initalized the `s1` we got information on both the
  `stack` and the `heap`. that the `s1` is the owner of this string.
  - the information that `s1` will be created as:
    - allocation of memeory on the heap of the string including `capcity`, `length` of the `String`
        ```rust
        `h`
        `e`
        `l`
        ...
        ```
    - while on  the stack it will keep the `ptr` the pointer that refer to this data as
        ```rust
        s1:{ptr: 0xe023}
        ```
- we passed the ownership to the `s2` and become the owner of the data (`String` value).
    - similary, now we have another pices of information as, we use same allocation that we have
    - keep another `ptr` that refer to same data, the string as
        ```rust
        s2:{ptr:0xe023}
        ```
- Once, we `println!` the `s2` which is a `macro`, the `s2`, will try to free
  as it goes out of the scope of this macro, taking with him the information
  that allocated on the heap,
- again, the `s2` want to also free same thing as we passwed same ownership to
  this variable, therefore, it will cause a conflict, and cause the compiler to complain and crash.

```shell
#    Hey, can I borrow that for a while?     Sure! But I have some rules I need to follow
#          +----------------- +               +----------------- +
#          |   Borrower       |               |     Owner        |
#          +----------------- +               +----------------- +
```

```rust
fn main(){
        let s1: String = String::from("Hellow World! ");
        println!("{}", s1);
        func1(s1); // <- func1 becomes the owner of the s1 on the heap, and you cannot use s1 anymore.

        println!("{}", s1);

    }

fn func1(s:String){
    println!("From func: {}", s);

    }
// How to fix
// we can use the (&) to take the reference to a string not an owenership of a string

func1(&s1)

```



## RFERENCES
- Check the script in ch01/ownership_borrowing.
- [Ownership & Borrowing | Rust Tutorials](https://www.youtube.com/watch?v=q2UnbA2dkc8&list=PLkO5ggdQuRaaeFke7nWS4ajhFVZ1biE7_&index=4)















