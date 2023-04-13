# Rust Life Time
- Difficult topic to learn
    - Even for seasoned developers.
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

- Now,  we will introduce the `lifetime` specifier.
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
- Rust can guarantee that the return reference will live long enough ifor downstream code to properly use it.
- Scope PROVIDING the reference is the same exact scope that will be RECEIVEING the result output.
- Meaning, that `param_1` reference scope is same as the return scope annotated with `-> &i32`.


















