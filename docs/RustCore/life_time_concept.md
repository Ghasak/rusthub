# Rust Life Time

- Difficult topic to learn
  - Even for seasoned developers.
- Summary of lifetime:
  - Each variable in Rust has a lifetime, which is the scope in which that
    variable is valid.
  - A lifetime is denoted by an apostrophe (') followed by a short name, like
    'a, 'b, etc.
  - A lifetime annotation is used to describe the relationships between the
    lifetimes of multiple variables. For example, if a reference to one
    variable is stored inside another variable, the lifetimes of the two
    variables must be related in some way.
  - Lifetime elision is a set of rules that allows the compiler to infer the
    lifetimes of references in many common cases. This can simplify code and
    reduce the need for explicit lifetime annotations.
  - The Rust borrow checker enforces the rules of lifetimes to prevent memory
    safety issues such as use-after-free errors.
  - The 'static lifetime refers to data that lives for the entire lifetime of
    the program. This is commonly used for string literals and other static
    data that is baked into the binary at compile time.

## Investigate these rules:

- [**GRule-1**]: It seems that we cannot return a reference for a variable created
  inside a funciton. As this variable will be dropped by reaching to the end of
  the function.

- [**GRule-2**]: `see Example -7`: The passed reference types in a function
  should be used as a placeholder to return `&str`, instead of using inner
  variable defined inside the function. It is almost immpossible to create a
  variable inside a function and return its slice as it will die when reach to
  the end of the scope of the function that it lives inside of it.

## Life time concept

- `lifetimes` are really about insuring memory doesn't get cleaned up before a
  reference can use it.
- Transformaing an ownership, is not a problem, but using reference, then you
  get this lifetime problem.
- `lifetime` is about references. and the references goes out of the scope.

```rust
let a;
{
    let b = String::from("Howday");

    a = &b;
}
println!("{a}" );
```

You, will get a complier error says:

```rust
`b` does not live long enough borrowed value does not live long enough.
```

- Notice that `a` is reference for nothing. as `b` is dropped.

### Example -1

- Missing Lifetime specifier this function's return type contains a borrowed
  value.but there is not value to be borrowed from.

```rust

fn get_int_ref()-> &i32{
    let a = 1;
    &a
}
```

- Rust complier knows, that since we didn't specifiy references in the param of
  the function.
- And, we created the reference internally (i.e., the borrowed reference would
  have to origiiinate from isndie the function), from the variable `a`.
- `a` will be dropped once its reach to the '}' of the end of the function,
  while the reference is return as `&i32`.
- Memeory gets cleanuped before the downstream code has a chance to use it.
- The life of the memeory we're returnign is not long enough if I return the
  ownership instead of reference, then it works as expected.

```rust
fn get_int_ref()-> i32{
        let a = 1
        a
    }
```

### Example -2

- Now, we will introduce the `lifetime` specifier.

```rust
fn main(){
    let some_int_var : i32 = 10;
    let result_ref = get_int_ref(&some_int_var);
    println!("{result_ref}");
    }

fn get_int_ref(param_1: &i32)-> &i32{
    param_1
}
```

- Notice, that we don't have an issue with the function `get_int_ref` why?
- Rust can guarantee that the return reference will live long enough ifor
  downstream code to properly use it.
- Scope PROVIDING the reference is the same exact scope that will be RECEIVEING
  the result output.
- Meaning, that `param_1` reference scope is same as the return scope annotated
  with `-> &i32`.

### Example -3

Here, I do the same as above, but we will get very amazing idea which is

- Rust lifetime is to specify the life of the reference, but it will not
  enforce it, meaning, once the reference goes to end of the scope it will be
  dropped, even though we say that it should live like the input scope.

```rust

pub fn life_time_concept_fn() {

    let some_int_var : i32 = 10;
    let reference_int_val = &some_int_var;
    //let result_ref: &i32;
    {
       let  result_ref: &i32 = get_int_ref(reference_int_val );
    }
    println!("{result_ref}");

}

/// This function is used for testing purposes only.
fn get_int_ref<'a>(param_1: &'a i32)-> &'a i32{
    param_1
}
```

- You will get a complier error says `cannot find the value result_ref in this scope not found in this scope.`
- This means that `result_ref` is only valid within the inner block and does
  not exist outside of it. Therefore, when you try to print `result_ref`
  outside of the inner block using `println!("{result_ref}");`, the Rust
  compiler cannot find it in the current scope and throws an error.
- [**OPENAI-CHATGPT**] Rustlife time for reference is for specify how long the
  reference should live but not enforce it? right?

  - Yes, that's correct. Rust lifetime annotations are used to indicate to
    the compiler how long a reference should live. The Rust compiler will
    enforce these annotations and ensure that the lifetime of a reference is
    valid for the duration of its use. However, it does not guarantee that
    the reference will actually live that long, as it is ultimately up to the
    program's execution to determine the lifetime of the reference.
  - You can find more information about Rust lifetimes in the official Rust
    documentation. These resources provide detailed explanations, examples,
    and best practices for working with Rust lifetimes. Here are a few links
    that you may find helpful:
    - The Rust Programming Language book:
      - [Rust language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
      - [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
      - [Rustonomicon](https://doc.rust-lang.org/nomicon/lifetimes.html)

- To fix the compliation error you can use

```rust

pub fn life_time_concept_fn() {

    let some_int_var : i32 = 10;
    let reference_int_val = &some_int_var;
    let result_ref: &i32;
    {
       result_ref = get_int_ref(reference_int_val );
    }
    println!("{result_ref}");

}

/// This function is used for testing purposes only.
fn get_int_ref<'a>(param_1: &'a i32)-> &'a i32{
    param_1
}
```

## Example -4

- The above example works, as we pass a `function` that return the reference
  with the `lifetime` specifier. We cannot compile the following code if its
  direct reference, as it will be considered as a dangling reference.
- The problem with this code is that lifetime_x is a reference to the value y,
  which is defined inside the inner block. When the inner block ends, y goes
  out of scope and its memory is deallocated. Therefore, lifetime_x becomes a
  dangling reference, pointing to memory that is no longer valid. When you try
  to print the value of lifetime_x outside the inner block, the compiler will
  raise an error because it is an invalid reference.

```rust

pub fn life_time_concept_fn() {
    let lifetime_x;
    {
        let y = 42;
        lifetime_x = &y;
        println!("{:?}", lifetime_x);
    }
    // lifetime_x is invalid here
    // let's print its value in the inner block instead
    println!("{:?}", lifetime_x);
}
```

## Example -5

Sometimes you want to return a `&str` which can be used for testing. This
`&static` is going to live all the way till the program is ended.

```rust
/// Return a static life time
/// This function return a string (on stack) which will live all the way until the program end.
fn testing_function() -> &'static str{
    let my_str_string: &str = "This is just  a test";
    my_str_string
}

```

## Example -6 From Byte to &Str and vice versa

Let's take this example to confim about our understanding to the references.
Assume you have the following string, you want to converate it to `bytes` first
then back to `&str` again.

- Notice as we used `str_a_as_byte` in the `converting_byte_to_str` function,
  it causes no problem as we have single parameter and we use it to return it
  as '&str'. We didn't generated a variable inside this function, this might be
  the proof for `GRule-2`.

```rust
let my_senence_str: &str = "Ghasak How are you dude!";
let output: &[u8] = checking_bytes(my_senence_str);

println!("{output:#?}");
let output: &str = converting_byte_to_str(output);
println!("{output:#?}");
```

```rust
/// From &str to bytes
fn checking_bytes(str_a: &str) -> &[u8] {
    let output = str_a.as_bytes();
    output
}
/// From bytes to &str
fn converting_byte_to_str(str_a_as_byte: &[u8]) -> &str {
    match std::str::from_utf8(str_a_as_byte) {
        Ok(s) => s,
        Err(_) => "",
    }
}
```

## Example -7 Generate interal string refernece

The following code shows the ability to get a reference on the `Heap` for a
`String` object. Then we return a `&'a str`, notice we can aslo return
`&'static str` instead.

- The issue with the code is that the output variable is a local variable that
  will be dropped at the end of the function scope. Therefore, returning a
  reference to it with &output would result in a dangling reference, which is
  undefined behavior.

- The alternative solution is to use `Box` with leak, or using `lazy_static` crate.

1. Using Box Leak.

```rust

fn another_testing<'a>(param_a: &'a str, param_b: &'a str) -> &'a str {
    let output = Box::leak(Box::new(format!("{}<->{}", param_a, param_b)));
    output
}
```

2. Using Box leak another format.

```rust

/// Another way to achieve that
fn can_be_done<'a>(param_a: &'a str, param_b: &'a str) -> &'a str {
    let output = format!("{param_a}<::>{param_b}");
    Box::leak(output.into_boxed_str())
}

```

3. Using `lazy_static`

```rust
use lazy_static::lazy_static;
lazy_static! {
    static ref OUTPUT: String = format!("{}<::>{}", "foo", "bar");
}
/// Using the Crate lazy_static
fn can_be_done_again(param_a: &str, param_b: &str) -> &'static str {
    &OUTPUT
}
```

4. This all could be avoided if we want simply allocat a variable on heap
   inside the function and return it, `ownwership in that case will be the return, which will be assigned to another variable if we want`.

- This example has nothing to do with `lifetime` rules.

```rust
fn obvious_solution(param_a: &str, param_b: &str) -> String {
    let concat_str: String = format!("{param_a}{param_b}");
    concat_str
}
```

## Example -7 Concat two &str references

No, there is no way to concatenate two &str references directly in Rust. The
&str type is an immutable borrowed reference to a string, and the + operator is
not defined for it because it requires ownership of the operands. However, we
can use the following way.

```rust
fn main(){
    let output : String = String::new();
    let output = crazy_idea("Wow", "another way",output );
    println!("{output}");

    }

#[allow(unused_assignments)]
fn crazy_idea<'a>(param_a: &'a str, param_b: &'a str, mut param_c:String)-> String{
    param_c = format!("{param_a}  {param_b}");
    param_c

}

```

## Continue - Rust Lifetime fromm Doug Milford

### Example -8 About reference - recap

- Lets create a function which will take no parameters, and return an `int` reference.

```rust
fn main(){

    }

fn get_int_ref()-> &i32 {
    let a = 1;
    &a
    }
```

- We will get a compiler error stats `missing lifetime specifier`. This
  function's return type contains a borrowed value, but there is no value for
  it to be borrowed from. consider giveing it a `static lifetime`: `'&'stattic'`

- So, if you are not going to return a `static` variable or `constant`, which
  is not for the sake of the arguement above we're not at the momenent, Rust
  compiler will have an issue with this.

- The compiler knows that since no references passed in the function
  `get_int_ref()`. That means the borrowed reference (returned value) would
  have to originate from inside the function. Rust compiler also knows any
  varaible created inside of a function will have their memeory cleaned up once
  it goes out of scope so before even call this function `Rust` knows that
  there's no way you're ever gonna get away with this half-baked scheme. It's
  similar to the situation where we have our inner scope. Memory gets cleaned
  up before the downstream code has a chance. The life to the memeory we're
  returning is not long enough. If instead return the ownership of the memory
  istead the reference as shown below it works as expected (i.e., there's no
  possible way co downstream is reading garbage data we just get used).

```rust
fn main(){

    }

fn get_int_ownership_back() -> i32 {
    let a = 1;
    a
    }
```

- Let's get back to our `get_int_ref` function. We are using an i-32 for
  demonstration, purposes I could have used any data type such as a string,
  vector, struct, enum ..etc. It doesn't matter the issue comes back to how
  long a reference can be expected to live regardless of memeory type or if the
  memery is on the stack or the heap. Stay focused on the fact that the issue
  due to `references`. Back to our example,

- We will modify the function to have a reference input as shown below. This
  time it will compile, because `Rust` guarantee that the return reference will
  live long enough for downstream code to properly use it. The scope that is
  providing the reference is the same exact scope that will be receiving result
  output.

```rust
fn main(){

    }

fn get_int_ref(param_1: &i32) -> &i32{
        param_1
    }
```

- Lets create an integer with the main function. We'll call our function
  passing in a reference to our variable the result data is just a reference to
  our original data. Rust can logically see that the scope of the input
  parameter `some_int_var` is within the same scope of output `result_ref`,
  since they both in same scope, it can guarantee the memeory is still valid at
  this point in other words, both are accessing memeory that lives to the end
  of the scope bracket of main function, hence the name `lifetime`.

```rust
fn main(){
        let some_int_var: i32 = 10;
        let result_ref = get_int_ref(&some_int_var);
        println!("{}" result_ref)
    }

fn get_int_ref(param_a: &i32) -> &i32{
        param_1
    }
```

- The way urst denotes a lifetime is with a generic symbol in angle brackets
  that has an apostrophe in front of it. It's convention to use the lowercase
  `a` for first lifetime `b` for second and so on, but you use whatever you
  like. It can even be multiple characters to be more descriptive
  `'some_lifetime`. The important piece is that as apostrophe in the front.
  Once you tell a function you're defining a lifetime you can apply to whatever
  input or output variables that apply.

```rust
fn main(){
        let some_int_var: i32 = 10;
        let result_ref = get_int_ref(&some_int_var);
        println!("{}" result_ref)
    }

fn get_int_ref<'a, 'b>(param_a: &i32) -> &i32{
        param_1
    }
```

- Let's apply for both our input and output as shown below. It says, I will
  define some generic lifetime and make sure my input and output comply with
  it. Meaning, the input memeory exists in the same scope is the output memory
  thus, live the same length of time. There's a caveat to that, but we'll get
  back to that later. `Note`: This notation used to be how every function was
  defined back in the old days. All references used to need an explicit
  lifetime. It's certainly more verbose that what you might be used to. It may
  take some time for us to adjust the syntax; even though you may have written
  your own functions without explicitly defining lifetimes for your references,
  they were there in the background, you just didn't know it. The Rust team
  realized that in many cases they can determine the lifetimes without making
  developer define it explicitly. So, they automated if for those situations
  and created a complier error in situation where it couldn't quite figure it
  out on its own.

```rust
fn main(){
        let some_int_var: i32 = 10;
        let result_ref = get_int_ref(&some_int_var);
        println!("{}" result_ref)
    }

fn get_int_ref<'a>(param_a: &'a i32) -> &'a i32{
        param_1
    }
```

- When you write a function (e.g., `some_function`) the complier will look to
  see what parameters if it sees one it creates a lifetime for and assigns it
  to its parameter. If it sees another reference it will add another lifetime
  `b`, and assign it to that parameter `param_2: &'b str` and so on and so
  forth. It will add as many lifetime definitions as it needs to cover the
  references in the parameters.

```rust
fn main(){
        let some_int_var: i32 = 10;
        let result_ref = get_int_ref(&some_int_var);
        println!("{}" result_ref)
    }

fn some_function<'a, 'b>(param_a: &'a str, param_2: &'b str) -> {

    }

fn get_int_ref<'a>(param_a: &'a i32) -> &'a i32{
        param_1
    }
```

- If you have any parameters that are not a reference it (e.g. `param_3: Vec<i64>`). It does nothing with those as those don't cause the issue related
  to referencing or cleaned up memeory.

```rust
fn main(){
        let some_int_var: i32 = 10;
        let result_ref = get_int_ref(&some_int_var);
        println!("{}" result_ref)
    }

fn some_function<'a, 'b>(param_a: &'a str, param_2: &'b str, param_3: Vec<i64>) {

    }

fn get_int_ref<'a>(param_a: &'a i32) -> &'a i32{
        param_1
    }
```

- If you have an output that is not a reference then all that work of compiler
  did to implicitly include lifetimes really wasn't helpful, because that will
  never have the issue we've described so no harm no foul.

```rust
fn some_function<'a, 'b>(param_a: &'a str, param_2: &'b str, param_3: Vec<i64>) -> String {
```

- If you do have it as a reference (as you can see `-> &str`), then the
  complier has to know what `lifetime` to assign to the output. That's where
  some of the trickiness comes into play and we will be touching upon further
  later.

```rust
fn some_function<'a, 'b>(param_a: &'a str, param_2: &'b str, param_3: Vec<i64>) -> &str {
```

- Instead we can write

```rust
fn some_function<'a>(param_a: &'a str, param_2: &'a str, param_3: Vec<i64>) -> &'a str {
```

- What about `static` or `constant` variables? can those be used a reference
  parameters?. Well static varaibles and constants are valid for the life of
  the program which means their lifetime is the `entirety` of the application,
  so I don't see why not rust has a special lifetime notation for that
  apostrophe `'static`. Let's create a constat at the top of the program as
  shown below. Then, we pass the constant to our function. Now, we will use
  that as an input and it is fine by rust, as it knows to treat this result
  with the same exact `lifetime` as the constant itself.

```rust
const SOME_TNT: i32 = 30;
fn main(){
    const SOME_INT: i32 = 30;
    let some_int_var = 10;
    let result_ref = get_int_ref(&SOME_INT);
    println!("{}", result_ref)

    }
fn get_int_ref<'a>(param_1: &'a i32) -> &'a i32 {
    param_1
}
```

- Lets remove now the constant, and add another parameter `param_2`, and pass
  it as its own, not a reference. Lets create another variable and pass it the
  function `some_int_var2`. It will complie and no issue with the `lifetime`
  (just a warning because our parameter is never used). So we didn't define a
  lifetime on param_2 nor should we. It's not a reference, if I even try to put
  a lifetime on it it won't complie `param_2: 'a i32`

```rust
fn main(){
    const SOME_INT: i32 = 30;
    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, some_int_var2 );
    println!("{}", result_ref)

    }
fn get_int_ref<'a>(param_1: &'a i32, param_2:i32) -> &'a i32 {
    param_1
}
```

- ['G Note'], Lets make it return an `Option`

```rust

fn get_int_ref<'a>(param_1: &'a i32, param_2: i32) -> Option<(&'a i32, i32)> {
    if param_2 > 10 {
        Some((param_1, param_2))
    } else {
        None
    }
}
// Other forms we can use
fn get_int_ref<'a>(param_1: &'static i32, param_2: i32) -> Option<(&'static i32, i32)>
// or
fn get_int_ref(param_1: &i32, param_2: i32) -> Option<(&i32, i32)>

```

- What do you think will happend if the second parameter was a reference
  instead of a non reference. We remove the `lifetime` and just make it as a
  reference. You will see that `param_2` is not the one being returned and thus
  it never has conflict with the output lifetime. Therefore, we currently
  don't need to explicitly state the lifetime for `param_2`. But, it does have
  a lifetime defined in the background, even though as the developer, we
  haven't explicitly defined it.

```rust
fn main(){
    const SOME_INT: i32 = 30;
    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, &some_int_var2 );
    println!("{}", result_ref)

    }
fn get_int_ref<'a>(param_1: &'a i32, param_2:&i32) -> &'a i32 {
    param_1
}
```

- Lets create a second lifetime called `b` and apply to our second parameter.
  This is how the complier understands it if we didn't explicitly define the
  lifetime on the `param_2`. Here we have a `rule-1`: **For every input
  reference that doesn't have an explicitly defined lifetime, it will be given
  it's OWN lifetime by Rust in the background**.

```rust
fn main(){
    const SOME_INT: i32 = 30;
    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, &some_int_var2 );
    println!("{}", result_ref)

    }
fn get_int_ref<'a, 'b>(param_1: &'a i32, param_2:&'b i32) -> &'a i32 {
    param_1
}
```

- Lifetime let's twist the knife little if it's possible for `parm_2` to be
  return that complicates the story. Let's create an if statement and return
  back the greater of our two parameters. We will get a lifetime mismatch
  compiler error, it's true the output has one lifetime denoted by `a` but at
  some point we're returning a lifetime denoted by `'b`. Rust can no longer
  guarantee that the code downstream will be ok. The second lifetime parameter
  could be bigger [**b >= a**] or smaller [**a < b**]. There is no way for Rust
  to know. The output variable could potentially outlive the reference lifetime
  which is a problem.

```rust
fn main(){
    const SOME_INT: i32 = 30;
    let some_int_var = 10;
    let some_int_var2 = 20;
    let result_ref = get_int_ref(&some_int_var, &some_int_var2 );
    println!("{}", result_ref)

    }
fn get_int_ref<'a, 'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}
}
// We get compliter error says
// lifetime may note live long enough consider adding the following bound: `'b: 'a`
```

- One possible solution is to tell the complier that `b` has to live at least
  as long as `a`. The way we do that is to put a `'b :'a`. This is called
  `lifetime subtyping`, it works just fine.But, it's a little verbose and it's
  overkill for most your needs as a developer we can look at the calling code
  and say yeah our two input parameters are withinn the same scope the result
  should be fine either way as the inputs `some_int_var` and `some_int_var2`
  within the same scope (`lifetime`).

```rust
fn get_int_ref<'a, 'b: 'a>(param_1: &'a i32, param_2:&'b i32) -> &'a i32 {
```

- But, Rust wants to guarantee the function is always valid, not just the code
  you're currently looking at. To do that, we simply ensure `param_1` and
  `param_2` are of the same lifetime. We will get rid of the `'b` lifetime, and
  ensure all has same lifetime which is `'a`. Now, the `lifetime` are
  guaranteed to meet my needs this may seems like it forces everything to have
  the same lifetime for calling code parameters, **not really** if you have the
  same lifetime definition across several parameters **think of it as
  minimum**, **Rust uses the MINIMUM of the passed in lifetime for any given
  call** which gives it a bit more flexibility.

```rust
fn get_int_ref<'a>(param_1: &'a i32, param_2:&'a i32) -> &'a i32 {
```

- Let's do realistic example that many people run into. Often, developers need
  to compare one or more string slices. So, they happily create a function that
  accepts two string slices. Let's say we return the best one, then we get a
  complier error says `explicit lifetime required in the type of param_1`,
  `lifetime 'static required rustc (E0621)` `lifetime static required`.

```rust
fn get_str_re(param_1: &str, param_2: &str) -> &str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}
```

- The solution of `lifetime` specifier is created to eliminate the potential
  dager of the `dangling reference`. Even though the developer can see in his
  code that the two parameters passed are having similar scopes so they
  shouln't have to worry about lifetime types rest. `Rust` wants to ensure the
  function definition itself is okay, not only the current code we are
  developing. Often, the solution is just attack on a explicit `lifetime` and
  be done with it.

```rust
fn get_str_re<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}
```

- So, you might be wondering why `Rrust` doesn't automatically assign the same
  lifetime to all parameters, wouldn't that be easier, well, for the give
  example the answer is yes, and often time, this is how you can fix lifetime
  issues. But, functions will take multiple reference parameters most of which
  don't get return as an output. You would have to write much more lifetime
  definitions if `Rust` defaulted to the same lifetime for each of those. The
  reason lifetimes is considered a specialized topic in the first place, is
  because `Rust` did a good job at automating a vast majority of it for you.
  And they could still come up with ways to automate even more in the future.
  They've only automated the low-hanging fruits at this point. You are just
  filling in the gaps that Rust couldn't figure out on its own from its default
  settings perhaps, lifetime will become fully automated, who knows.

- Let's do some cleanup for now. and dealing with some cases. What lifetime
  would `Rust` infer from this? Since `lifetime` has to deal with references
  only,`lifetime` don't apply because there are no reference inputs or output.

```rust

fn test_1(param_1: Vec<f64>) -> Vec<f64>{
        param_1
    }
```

- What about this one, we have a single parameter reference but a non reference
  for return type. Although you have a reference input, lifetimes aren't an
  issue because there is no reference output. That dosn't mean the lifetime
  doesn't exist, the complier still sees it like so, it just has no effect
  on the output lifetime is and thus doesn't force you to explicitly define
  it.

```rust
#[allow(dead_code)]
fn test_2(param_1: &Vec<f64>) -> vec<f64>{
        param_1.clone()
    }
```

- What if we had a reference output but no reference input (or lifetime
  problem). In this case, it is missing alifetime specifier.

```rust
#[allow(dead_code)]
fn test_2(param_1: Vec<f64>) -> &Vec<f64>{
        param_1
    }

```

- So let's give it that and see what it dose. It says, cannot return reference
  to function parameter param_1 return a reference to data owned by the current
  function. This is same problem that we delt with when there is no parameter
  since Ownership of the memory gets transfered to the function. That memeory
  will now get cleaned up to hits the end bracket. We can't pass back a
  reference or else the receiving variable will be referencing garabage
  memeory. `lifetime` doesn't apply in that case because there are no reference
  inputs, the caveat is that if we want to return a static lifetime reference
  we could do that.

```rust
#[allow(dead_code)]
fn test_3<'a>(param_1: Vec<f64>) -> &'a Vec<f64> {
 // here will be dropped the param_1 as its ownwership transfered to the function test_3,
    &param_1
}
```

- In that case you have two options, return ownership of the variable

```rust
fn test_3(param_1: Vec<f64>) -> Vec<f64>{
        param_1
    }
```

- Or, ensure lifetime of input reference similar to output reference lifetime

```rust
fn test_3<'a>(param_1: &'a Vec<f64>) -> &'a Vec<f64>{
    param_1

    }
```

- Now, lets create more complicated example, but should be same. The following
  example, shows that we will get a compile errors. The complier sees two
  parameters as references `paramp_2` and `param_3`

```rust
fn main(){

    }

#[allow(dead_code)]
fn test_4(param_1: i32, param_2: &str, param_3:&str, param_4: f64)-> &str{
    if param_1 == 7 && param_4 > 10. {
                param_2
        }else{
                param_3
            }

    }
```

- So we create two lifetime placeholders (usually these are in background), It
  will apply `lifetime a` to `param_2` and apply `lifetime b` to `param_3`. If
  we have third parameter as a reference it would create a lifetime `lifetime 'c` and apply it to that and so on and so forth. For the output it doesn't
  know what the heck it wants to do with that, so we'll leave it for now. This
  is what the complier believes your intention to be. Vast majority of the
  cases you will say, we don't need to support different lifetimes for
  different parameters.

```rust
fn test_4<'a, 'b>(param_1: i32, param_2: &'a str, param_3:&'b str, param_4: f64)-> &str{
```

- In that case, I'm gonna explicitly define a single lifetime and apply it to
  everything, this is what you as the developer would have written to force the
  lifetimes to be the same. Rust just had some edge cases that you may not have
  intended and realized there'd be a problem henece the complier error.
  Therefore, you just need to step in and say I only want to support the same
  lifetime or whateveer lifetimes fits our needs. Everytime a reference
  parameters that didn't have a chance of being returned then I'd be ok to not
  putting in a lifetime specifier on them, it's okay that they would have their
  own lifetime and the default that Rust provided will be just fine, In this
  example all the input reference parameters have the chance of being returned
  and that's why we have explicitly forced the lifetime to be the same. Rust
  will ensure that whatever your scheme memory issues won't occur. **rule-2:
  Any reference parameter that is NEVER returned as the output, the Rust
  provided default
  lifetime is fine**.

```rust
fn test_4<'a>(param_1: i32, param_2: &'a str, param_3:&'a str, param_4: f64)-> &'a str{
```

- There is special case that the `Rust` will handle automatically and that's if
  you have a single input reference and an output reference.You still could
  have more non reference inputs and this would still apply. It doesn't really
  matter what you would do in the body so I'll just return pram a `Rust` still
  applies lifetimes to the parameters like it did before but if you have only
  one input reference it will enforce the output reference to have the same
  lifetime.

```rust
#[allow(dead_code)]
fn test_2(param_1: &str) -> &str{
        param_1
    }
```

- There's no confusion into the siutation, so you would never have to
  explicitly define lifetimes here `Rust` as shown below. `Rust` can safely
  assume this is what your intention was.

```rust
#[allow(dead_code)]
fn test_2<'a>(param_1: &'a str) -> &'astr{
        param_1
    }
```

- Let's clean up to explore vectors. I'll create a function that takes in a
  vector slice reference or array reference. we will return a `slice` of this
  array. Lets call also the function in the main by creating an a vector and
  pass it to our function. [**Read Slice &[i32] as a reference vs
  &Vec?**](../questions.md).

```rust
fn main(){
    let a: Vec<i32> = vec![1,2,3,4,5];
    let result = get_vec_slice(&a);
    println!("{result:#?}")

    }
fn get_vec_slice(param_1: &[i32]) -> &[i32]{
        &param_1[0..2]
    }
```

- We are not return the full vector, instead we return a slice which is
  perfectly legitimate if the entir vector has a certain lifetime, then all its
  elements have that same lifetime, even though you're only returning a subset
  or slice, Rust will compile just fine.
- Let's augment our function by adding more referenc of the same type. Put some
  logic in the function body.

```rust
fn main(){
    let a: Vec<i32> = vec![1,2,3,4,5];
    let result = get_vec_slice(&a);
    println!("{result:#?}")

    }
fn get_vec_slice(param_1: &[i32], param_2: &[i32]) -> &[i32]{
    if  param_1.len() > param_2.len(){
        &param_1[0..2]
        }else{
        &param_2[0..2]
            }
    }
```

- You can see the complier cannot determine what the lifetime should be it's
  current assuming program one has one lifetime (i.e. is it 'a or 'b should be
  returned?). What the output lifetime? We need to make the `lifetime` similar
  as we did before. Do we really want `param_1` and `param_2` to have different
  `lifetime`? let's tell teh complier is not really necessary to support to do
  is call the function with valid data and we're off to the races.

```rust

fn get_vec_slice<'a>(param_1: &'a[i32], param_2: &'a[i32]) -> &'a[i32]{
    if  param_1.len() > param_2.len(){
        &param_1[0..2]
        }else{
        &param_2[0..2]
            }
    }
```

- We did this for `Vector` in both of their types on `stack` and on `heap`. As
  a reminder, strings are just a vectors of `u8` data type. So the same concept
  applies. String parsing is often where people run into issue with lifetimes,
  but now we know better.

```rust
fn main(){
    let s: &str = "This is my given function";
    let o = get_string_slice(s);
    println!("{o:#?}")
    }

fn get_string_slice(param_1: &str) -> &str {
    let k: usize = param_1.len();
    if k > 2 {
        let half: usize = k / 2;
        &param_1[0..half]
    } else if k == 1 {
        &param_1[0..1]
    } else {
        param_1
    }
}
```

- Now, we will talk about the `static` lifetime. `static`: means lifetime that
  lasts the entire program. `Constant` are static by their nature. You can also
  have a static variables (by using the static keyword, but they will behave
  the same). We will create some constants and a function that return a static
  variable. `Rust` reserved the lifetime notation of apostrophe static, you
  don't need to define it in angular brackets like you would for apostrophe. It
  just part of the language. That's the exception that a reference output needs to

```rust
const SOME_CONST_A : &str = "I'm a constant!";
const SOME_CONST_B : &str = "I'm a constant, too!";

fn main(){
    let output:&str = some_func();
    println!("{output:#?}")
    }


fn some_func() -> &'static str{
    SOME_CONST_A
    }
```

- It is not limited to the `output` only, we can have a static input parameter
  as well.

```rust

fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
    if param_1.len() > param_2.len() {
        SOME_CONST_A
    } else {
        SOME_CONST_B
    }
}

```

- You can also return a reference, sinc the const will never die, we don't do
  that and its fine to pass the ownership for each to their param_1 and
  param_2.

```rust
let output: &str = some_func(&SOME_CONST_A, &SOME_CONST_B);
```

- Defining a function with `'static` lifetime, means, all the passed parameters
  has to live the entir of program. This assumption is very restricts, instead
  we can use identical lifetime `'a` which will work fine for any case
  scenario.

```rust
fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
//change to
fn some_func<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
// Now even when you pass another variable that is not 'static
let var = String::from("variable passed");
// this was not allowed as var has less lifetime and not static.
```

- Let's talk now about `lifetime` with `Generic`. We will use PartialOrd so
  that we caaan compare if `T` items are greater or smaller than other `T`
  items.

```rust
fn get_smaller<T: std::cmp::PartialOrd>(param_1: &T, param_2: &T) -> &T{
    if param_1 > param_2{
        param_1
    }else{
        param_2
    }
}
```

- You can see the same issue, we need to specify the `lifetime`. We have solved
  this before, just we have `generic` dosn't make it any different. We define
  the `lifetime` in the same `Geenric` angle brackets. Notice that we must
  provide both `param_1` and `param_2` generic same type since we use `T` for
  both.

```rust
fn get_smaller<'a,T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T{

```

```rust
fn main(){
    let var_a = 10;
    let var_b = 20;
    let var_k: &str = "wow";
    let var_w: &str = "again";
    let output_1 = get_smaller(&var_a, &var_b);
    let output_2 = get_smaller_2(&var_w, &var_k);

    println!("{output_1:#?}");
    println!("{output_2:#?}");
    }

//fn get_smaller<T: std::cmp::PartialOrd>(param_1: &T, param_2: &T) -> &T{
fn get_smaller<'a, T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

fn get_smaller_2<'a, T >(param_1: &'a T, param_2: &'a T) -> &'a T
where T: std::cmp::PartialOrd{
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}
```

- Lets talk now about `Structs`. `Struct` also have lifetimes, same like
  `function`, Rust `struct` concerns only wiht references. If you have a field
  with a reference type, such as `some_reference_data` below, then, we get our
  faimilar complier error. Since, `MyStruct` has its own memeory but references
  some memory outisde of itself you need to make sure the reference lives long
  enough where it doesn't leave the `MyStruct` referencing bad memeory.

```rust
struct MyStruct{
        some_data : Vec<i32>,
        some_reference_data: &Vec<i32>,
    }

fn main(){

    }
```

- So, lets give it a lifetime and apply it to the reference (similar to
  functions). Seems trivial, and wondering why we need to go through the ceremony!

```rust

struct MyStruct<'a, 'b>{
        some_data : Vec<i32>,
        some_reference_data: &'a Vec<i32>,
        some_reference_data_2: &'b Vec<i32>,
    }

fn main(){

    }
```

- We can say, we have two references but they don't have to be the same
  lifetime. We can also use the `subtype` as we did before. We usually don't
  have a need to syntax in this form, because usually if We have two references
  they will be within the same scope and will be autotmaically have same
  lifetime. However, if you see this syntax you will know what it says.

```rust
struct MyStruct<'a, 'b: 'a>{
        some_data : Vec<i32>,
        some_reference_data: &'a Vec<i32>,
        some_reference_data_2: &'b Vec<i32>,
    }
```

- Often the solution based on the needs is to unified their lifetime, if both
  `some_reference_data` and `some_reference_data_2` are both created in same
  scope.

```rust
struct MyStruct<'a>{
        some_data : Vec<i32>,
        some_reference_data: &'a Vec<i32>,
        some_reference_data_2: &'a Vec<i32>,
    }
```

- Typical example with Structs

```rust

#[derive(Debug)]
struct MyStruct<'a> {
    some_data: Vec<i32>,
    some_reference_data: &'a Vec<i32>,
    some_reference_data_2: &'a Vec<i32>,
}

/// # Demonstration Function
/// ## Function Highlights
/// ### Input
/// The function require some inputs for demonstration.
pub fn life_time_concept_fn() {
    let v1: Vec<i32> = vec![10, 20, 30];
    let v2: Vec<i32> = vec![40, 50, 60];

    {
        let obj = MyStruct {
            some_data: vec![1, 2, 3, 4, 5],
            some_reference_data: &v1,
            some_reference_data_2: &v2,
        };
        println!("{:#?}", obj);
    }
}
```

- Here is a comparisonn I made, need to be reviewed.

| siutation                                   | Function                                 | structs                    |
| ------------------------------------------- | ---------------------------------------- | -------------------------- |
| Input one parameter vs one filed            | implicitly define lifetime               | explicitly define lifetime |
| Adding another param or field               | trigger new lifetime assignment          | same                       |
| potential reference (output param or field) | conflict with lifetime, need to specifiy | same                       |

## REFERENCES

- [Rust Lifetimes](https://www.youtube.com/watch?v=1QoT9fmPYr8&t=229s)
