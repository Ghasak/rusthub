# Numerical calcuations in Rust
## Content

### Vectorizing

For the developed numerical analysis, we can use the `ndarray` in Rust along with the other carts from rust.
To enable the `ndarray`, add,

```toml
rayon = {version = "1.6.1", optional = true}
ndarray = {version = "0.15.6", features = ["rayon"]}

```
This will add the `rayon` to speed parallel comutations, first you need to call it.

```rust
use ndarray::prelude::*;
use ndarray::parallel::prelude::*;

```
Once, we create the array, if we can apply some arithmetic, we can clone it, as the `c` will be the owner of `a`.
```rust
    let a = Array::linspace(0., 63., 64).into_shape((4, 16)).unwrap();
    let c = a.clone().mapv_into(|v| f64::powf(v, 2.0));
    println!("{:#?}", a);
    println!("\n ******", );
    println!("{:#?}", c);

```


