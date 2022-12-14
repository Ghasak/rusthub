# rusthub
Thoughts and ideas on Rust lang, Including insights from many references and
resources.


## To do
- [ ] class Based OOP vs Traits (can be found
  [here](https://www.youtube.com/watch?v=m_phdVlkr6U&t=158s))
    - [ ] Relationship of other languages to Rust for example, `oop` vs
      `traits`.
- [x]  Module and Crates for projects
    - including `super`, `self` and `crate` syntax
- [x] life-time, check [here](https://www.youtube.com/watch?v=1QoT9fmPYr8)
- [ ] Adding `JupyterNotebook` support for `Rust`,
- [ ] Chaining methods, iterators and closures.

## How to use

Run your main programming at root directory of crate using, to check the
backgrace while compliation.

```rust
RUST_BACKTRACE=1 cargo run --quiet
```
## Tools for investigating the project skelton in Rust
You will need a `Crate` called `cargo-modules` which will allow us to visulize
the our developed crate modules tree.
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
To see other features you can use `cargo modules generate tree  --help`.

- Now, lets even the `enum` `struct` ..etc using


```rust
╭─ gmbp   GMacBookPro on ~/Desktop/devCode/rust_fundamentals/rusthub   
├─ﮧ INSERT  17h52m|main !6 ?3
╰─ cargo modules generate tree --with-types

crate rusthub
├── mod concepts: pub(crate)
│   ├── mod ch01: pub
│   │   ├── mod common_collections: pub
│   │   │   ├── fn common_collections_fn: pub
│   │   │   ├── fn string_to_static_str: pub
│   │   │   └── fn type_of: pub
│   │   ├── mod experimental_ideas: pub
│   │   │   └── fn experiment_sum_fn: pub
│   │   ├── mod memeory_investigating: pub
│   │   │   └── fn investigate_memeory_allocation: pub
│   │   └── mod ownership_borrowing: pub
│   │       ├── fn about_owner_ship_concepts: pub
│   │       ├── fn func1: pub
│   │       └── fn square: pub
│   └── fn create_text: pub
└── fn main: pub(crate)
```

## Profiling your project

```rust
cargo install flamegraph
```
```rust
flamegraph -o my_flamegraph.svg -- ./target/debug/rust_learning_hub
```

## Documentation
I use the both the defintion like `///` and the `//?` on the main page. I also
not include the dependencies.
```rust
cargo docu --no-deps --open
```

## Auto Run Using Cargo
You can use `cargo-watch` which will keep on running our project and working in background.
- First install it on global `rust` at the `~/.cargo/` using:
    ```rust
    cargo install cargo-watch
    ```
- Open a new `termina` and run:
    ```rust
    cargo-watch -x run
    ```
- But, to push it further, we can clear the `console` as well, using
    ```rust
    cargo-watch -c -x run --quiet


    ```

## Libraries to be inlcuded
1. [Neuronika Optim](https://docs.rs/neuronika/0.1.0/neuronika/optim/index.html)

