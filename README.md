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
- [ ] life-time


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
To see other features you can use `cargo modules generate tree  --help`
- Now, lets even the `enum` `struct` ..etc using:


```rust
├─ﮧ INSERT  1h28m|main !8 ?1
╰─ cargo modules generate tree --with-types

crate rust_learning_hub
├── mod concepts: pub(crate)
│   ├── mod ch01: pub
│   │   ├── mod common_collections: pub
│   │   │   ├── fn common_collections_fn: pub
│   │   │   ├── fn string_to_static_str: pub
│   │   │   └── fn type_of: pub
│   │   ├── mod experimental_ideas: pub
│   │   │   └── fn experiment_sum_fn: pub
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
