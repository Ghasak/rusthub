# General Facts
<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [General Facts](#general-facts)
    - [Intro for Rust in General](#intro-for-rust-in-general)
    - [Comparing this with C++:](#comparing-this-with-c)
    - [Why compling takes so long time?](#why-compling-takes-so-long-time)

<!-- markdown-toc end -->

## Intro for Rust in General

Rust is a systems programming language focused on safety, concurrency, and
performance. It achieves memory safety without garbage collection through a
system of ownership, borrowing, and lifetimes. Here's how Rust generally works:

1. **Ownership**: Every value in Rust has a variable that's its owner. There
   can only be one owner at a time, and when the owner goes out of scope, the
   value is dropped.

2. **Borrowing**: Instead of transferring ownership, Rust allows borrowing
   references to values. Borrowing can be mutable or immutable, and the borrow
   checker ensures that references are valid.

3. **Lifetimes**: Lifetimes specify the scope for which references are valid.
   They are a way of specifying the relationship between the lifetimes of
   various references in your code.

4. **Cargo**: Rust's package manager and build system. It handles dependency
   management, building, and publishing of Rust projects. `cargo run` compiles
   your code along with its dependencies fetched from crates.io and then runs it.
   When you use `cargo run` or `cargo build`, Cargo fetches dependencies, compiles
   everything, and then runs your code.

## Comparing this with C++:

- In C++, you can link dynamic or static libraries to your code, similarly to
  Rust's dependency management via Cargo. However, the management of these
  libraries is less integrated into the language and build system compared to
  Rust's Cargo.

- Rust's compile times can indeed be slower compared to C++, especially for
  large projects. This is due to several factors:
  - The borrow checker and ownership system require more analysis during
    compilation.
  - Dependency resolution and compilation through Cargo can add overhead.
  - Rust's strong emphasis on safety and correctness often leads to more
    complex optimization passes during compilation.

While Rust's compilation times can be longer, it's important to note that
Rust's compile-time safety checks often catch bugs that would only be
discovered at runtime in languages like C++. Additionally, Rust's performance
benefits from these safety guarantees often outweigh the longer compilation
times in the long run. Furthermore, ongoing improvements to the compiler and
tooling are continuously addressing Rust's compile-time performance.

## Why compling takes so long time?

In Rust, when you run `cargo run`, it does indeed compile not only your code
but also fetches and compiles all the code from the dependencies listed in your
`Cargo.toml` file. The entire dependency tree is resolved, fetched, and
compiled before your code is compiled and executed. This ensures that the
dependencies are compatible and integrated correctly with your code,
maintaining Rust's safety and integrity guarantees.

Comparing this behavior to C++, where you can attach dynamic or static
libraries to your code, in Rust, the process is more integrated and
standardized through Cargo. Cargo handles dependency management, ensuring that
all dependencies are fetched, compiled, and linked correctly. This integration
comes at a cost, as fetching and compiling dependencies can add overhead to the
compilation process, making Rust's compilation slower compared to C++ in
certain cases.

Rust's emphasis on safety, correctness, and dependency management contributes
to its slower compilation times. However, the trade-off is a language and
ecosystem that prioritize reliability and maintainability. While Rust's
compilation times may be slower, the benefits of safety, concurrency, and
performance often outweigh this drawback, especially in long-term projects
where the upfront investment in compilation time pays off in terms of reduced
debugging and maintenance efforts.
