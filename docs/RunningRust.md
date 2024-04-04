# How to run Rust

## Compile your binary

There are several ways to compile your programm.

```sh
cargo run --quiet
```

or

```sh
rustc src/main.rs && /target/debug/<binary_name>
# example
rustc src/main.rs && target/debug/rust_cli
# Correct way to send the binary to the location we know
rustc  src/main.rs  -o target/debug/output && target/debug/output
```

```sh
cargo build --quiet && ./target/debug/<binary_name>
```

## Adding Colors and Nerd fonts in Rust

1. Adding colors
   There are two ways

- Adding using the `carte` called `colored`

```rust
use colored::Colorize;

fn main() {
    println!(
        "{}, {}, {}, {}, {}, {}, and some normal text.",
        format!("Bold").bold(),
        format!("Red").red(),
        format!("Yellow").yellow(),
        format!("Green Strikethrough").green().strikethrough(),
        format!("Blue Underline").blue().underline(),
        format!("Purple Italics").purple().italic()
    );
}
```

- Using the `hexdecimal` representations

```rust
 println!("Now we have the following   \x1b[93mError -> \u{f013}\x1b[0m::-> greek = {:?}", greeks);

```

2. Adding Icons (nerd font)
   You can use `\u{0000}` following the hexa deciaml representations.

- [How do I print colored text to the terminal in Rust](https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust)
