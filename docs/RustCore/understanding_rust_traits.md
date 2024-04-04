# Understanding Rust Traits

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->

**Table of Contents**

- [Understanding Rust Traits](#understanding-rust-traits)
  - [Introduction](#introduction)
    - [1. What is a Trait?](#1-what-is-a-trait)
    - [2. Purpose of Traits:](#2-purpose-of-traits)
    - [3. Using Traits:](#3-using-traits)
    - [4. Deriving Traits:](#4-deriving-traits)
    - [5. Custom Designed Trait:](#5-custom-designed-trait)
    - [6. Example: `Describable` Trait:](#6-example-describable-trait)
  - [Example](#example)
  - [Custom Trait - Defines a custom trait `Describable`](#custom-trait---defines-a-custom-trait-describable)
  - [Default Trait From Standard Library](#default-trait-from-standard-library)
  - [Custom Derived Trait](#custom-derived-trait)
    - [Q&E Session](#qe-session)
      - [Q1 About the derive keyword](#q1-about-the-derive-keyword)

<!-- markdown-toc end -->

## Introduction

Finally, I can say that I have understood the concept behind the `derive` which
gives us a way to attach any trait (e.g., from the standard library of Rust,
from other packages or custom designed). Here, I will show the `custom designed trait` that I used to attached to a `struct` that I use to carry the information
of three fields.

- You need to review and read the implemation of the `rust trait` at `../../src/concepts/ch03/understanding_traits_concept.rs`.

Let's break down and explain the concept of traits, especially in the context of Rust.

### 1. What is a Trait?

A trait in Rust is a language feature that defines a set of behaviors or
functionalities that a type can implement. It allows you to define shared
behavior in an abstract way, independent of the specific data type.

### 2. Purpose of Traits:

- **Encapsulation of Functionality**: Traits encapsulate functionality and
  features, providing a way to define common behavior that can be shared among
  different types.
- **Code Reusability**: By defining common behavior in traits, you can reuse
  code across multiple data structures without duplicating implementations.
- **Polymorphism**: Traits enable polymorphism in Rust, allowing different
  types to exhibit similar behavior through trait implementations.

### 3. Using Traits:

- **Defining a Trait**: Traits are defined using the `trait` keyword followed
  by the trait name and a set of method signatures. These methods define the
  behavior that types implementing the trait must provide.

- **Implementing a Trait**: Types can implement traits by providing
  implementations for the methods defined in the trait. This is done using the
  `impl` keyword followed by the trait name and the method implementations.

### 4. Deriving Traits:

- The `derive` attribute in Rust allows automatic generation of trait
  implementations for certain traits, such as `Debug`, `Clone`, `Copy`, etc.
  However, it doesn't work for custom traits. For custom traits, you need to
  manually implement them for your types.

### 5. Custom Designed Trait:

- In Rust, you can create your own custom traits tailored to your specific
  needs. These traits can define behavior that is relevant to your application
  domain.

- By implementing a custom trait for a type, you can attach specific
  functionality to that type, allowing it to exhibit the behavior defined by the
  trait.

### 6. Example: `Describable` Trait:

- In your example, you have defined a custom trait called `Describable` with a
  method `describe()`. This trait encapsulates the behavior of describing an
  object.

- You then implement the `Describable` trait for the `Book` struct, providing a
  specific implementation for the `describe()` method. This allows instances of
  `Book` to be described using the `describe()` method.

- By using this custom trait, you can attach the `describe()` behavior to any
  data structure that implements the `Describable` trait, providing a consistent
  way to describe different types of objects.

In summary, traits in Rust provide a powerful mechanism for defining shared
behavior, promoting code reuse, encapsulation, and polymorphism.
Custom-designed traits allow you to tailor behavior to specific requirements,
providing flexibility and modularity in your codebase.

## Example

```rust
// Define a custom trait `Describable`
pub trait Describable {
    fn describe(&self);
}

// Implement the trait `Describable` for struct `Book`
impl Describable for Book {
    fn describe(&self) {
        println!("[INFO] Title: {}", self.title);
        println!("[INFO] Author: {}", self.author); // Fixed the typo here
        println!("[INFO] Pages: {}", self.pages);
    }
}

// Define a struct `Book`
pub struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    // Constructor method for Book
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book { title, author, pages }
    }
}

pub fn testing_trait_fundamental_concept() {
    let book = Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik & Carol Nichols".to_string(),
        500,
    );
    // Calling the describe method from the trait
    book.describe();
}


```

## Custom Trait - Defines a custom trait `Describable`

```rust
use std::fmt;

/// Define a custom trait `Describable`.
/// This trait provides a method `describe()` which prints information about the object.
pub trait Describable {
    fn describe(&self);
}

/// Implement the trait `Describable` for struct `Book`.
/// This implementation allows `Book` instances to be described using the `describe()` method.
impl Describable for Book {
    fn describe(&self) {
        println!("[INFO] Title: {}", self.title);
        println!("[INFO] Author: {}", self.author);
        println!("[INFO] Pages: {}", self.pages);
    }
}

/// Define a struct `Book`.
/// This struct represents a book with title, author, and number of pages.
pub struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    /// Constructor method for `Book`.
    /// Constructs a new `Book` instance with the given title, author, and number of pages.
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book { title, author, pages }
    }
}

/// Implementing the Debug trait for struct `Book`.
/// This allows the `Book` struct to be printed using the `{:?}` formatter.
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Book {{ title: {}, author: {}, pages: {} }}", self.title, self.author, self.pages)
    }
}

/// Function to test the fundamental concept of traits.
pub fn testing_trait_fundamental_concept() {
    // Create a new Book instance
    let book = Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik & Carol Nichols".to_string(),
        500,
    );
    // Print the Book struct using the Debug trait
    println!("{:?}", book);

    // Call the describe method from the trait to print information about the Book
    book.describe();
}
```

Explanation:

1. **`Describable` Trait**:

   - Defines a custom trait `Describable` with a single method `describe()`. This trait allows objects to be described.

2. **`Book` Struct**:

   - Represents a book with title, author, and number of pages.
   - Provides a constructor method `new()` to create a new `Book` instance.

3. **Trait Implementation**:

   - `impl Describable for Book`: Implements the `Describable` trait for the `Book` struct. This allows `Book` instances to be described using the `describe()` method.

4. **Debug Trait Implementation**:

   - `impl fmt::Debug for Book`: Implements the `Debug` trait for the `Book` struct. This allows the `Book` struct to be printed using the `{:?}` formatter.

5. **`testing_trait_fundamental_concept()` Function**:
   - Creates a new `Book` instance.
   - Prints the `Book` struct using the `Debug` trait.
   - Calls the `describe()` method from the `Describable` trait to print information about the `Book`.

Overall, this code demonstrates the usage of custom traits, as well as
implementing the standard library's `Debug` trait for printing custom data
structures. It showcases how traits can encapsulate behavior and provide a
consistent interface for interacting with objects.

## Default Trait From Standard Library

Let's modify the code to use `derive(Debug)` instead of manually implementing
the `Debug` trait for the `Book` struct. Then, we'll compare it with the
previous example using a table to illustrate the differences.

Here's the modified code using `derive(Debug)`:

```rust
/// Define a custom trait `Describable`.
/// This trait provides a method `describe()` which prints information about the object.
pub trait Describable {
    fn describe(&self);
}

/// Implement the trait `Describable` for struct `Book`.
/// This implementation allows `Book` instances to be described using the `describe()` method.
impl Describable for Book {
    fn describe(&self) {
        println!("[INFO] Title: {}", self.title);
        println!("[INFO] Author: {}", self.author);
        println!("[INFO] Pages: {}", self.pages);
    }
}

/// Define a struct `Book`.
/// This struct represents a book with title, author, and number of pages.
#[derive(Debug)] // Derive the Debug trait for the Book struct
pub struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    /// Constructor method for `Book`.
    /// Constructs a new `Book` instance with the given title, author, and number of pages.
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book { title, author, pages }
    }
}

/// Function to test the fundamental concept of traits.
pub fn testing_trait_fundamental_concept() {
    // Create a new Book instance
    let book = Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik & Carol Nichols".to_string(),
        500,
    );
    // Print the Book struct using the Debug trait
    println!("{:?}", book);

    // Call the describe method from the trait to print information about the Book
    book.describe();
}
```

Now, let's compare this version with the previous one in a table:

| Feature                   | Manual Implementation                        | `derive(Debug)` Implementation               |
| ------------------------- | -------------------------------------------- | -------------------------------------------- |
| `Describable` Trait       | Manually implemented                         | Manually implemented                         |
| `Debug` Trait for `Book`  | Manually implemented                         | Derived using `derive(Debug)`                |
| Implementation Complexity | Requires manual implementation of `Debug`    | Automatically derived by Rust using `derive` |
| Code Length               | Slightly longer due to manual implementation | Shorter due to automatic derivation          |
| Maintenance               | Requires updating `Debug` implementation     | No need to update `Debug` implementation     |

In summary, the main difference lies in how the `Debug` trait is implemented
for the `Book` struct. The manual implementation requires more code and
maintenance effort, while the `derive(Debug)` approach is more concise and
automatically generated by Rust.

## Custom Derived Trait

To achieve this, we need to manually implement the `Debug` trait for the `Book`
struct and provide a custom implementation of the `fmt` method. This custom
implementation will allow us to print the `Book` struct with custom messages
instead of the default format provided by `derive(Debug)`.

Here's how you can modify the code to achieve this:

```rust
use std::fmt;

/// Define a custom trait `Describable`.
/// This trait provides a method `describe()` which prints information about the object.
pub trait Describable {
    fn describe(&self);
}

/// Implement the trait `Describable` for struct `Book`.
/// This implementation allows `Book` instances to be described using the `describe()` method.
impl Describable for Book {
    fn describe(&self) {
        println!("[INFO] Title: {}", self.title);
        println!("[INFO] Author: {}", self.author);
        println!("[INFO] Pages: {}", self.pages);
    }
}

/// Define a struct `Book`.
/// This struct represents a book with title, author, and number of pages.
#[derive(Debug)] // Derive the Debug trait for the Book struct
pub struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    /// Constructor method for `Book`.
    /// Constructs a new `Book` instance with the given title, author, and number of pages.
    pub fn new(title: String, author: String, pages: u32) -> Self {
        Book { title, author, pages }
    }
}

/// Implementing the Debug trait for struct `Book`.
/// This allows the `Book` struct to be printed with custom messages.
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Book {{\n    Title: {},\n    Author: {},\n    Pages: {}\n}}", self.title, self.author, self.pages)
    }
}

/// Function to test the fundamental concept of traits.
pub fn testing_trait_fundamental_concept() {
    // Create a new Book instance
    let book = Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik & Carol Nichols".to_string(),
        500,
    );
    // Print the Book struct using the Debug trait
    println!("{:?}", book);

    // Call the describe method from the trait to print information about the Book
    book.describe();
}
```

In this implementation:

- We manually implement the `Debug` trait for the `Book` struct.
- The `fmt` method is provided with a custom implementation to format the
  `Book` struct with custom messages.
- Inside the `fmt` method, we use the `write!` macro to write the custom
  message with the `Book` fields.

With this modification, when you print a `Book` struct using the `{:?}`
formatter, it will display the custom message defined in the `fmt` method of
the `Debug` implementation.

### Q&E Session

#### Q1 About the derive keyword

- Can we remove the `#[dervie(Debug)]` from previous example?

  - Yes, including `#[derive(Debug)]` in the previous example would be considered
    extra verbose and unnecessary if you manually implement the `Debug` trait with
    custom formatting for the `Book` struct. This is because `#[derive(Debug)]`
    automatically implements the `Debug` trait for the struct with default
    formatting, which may not be what you desire if you want custom formatting.

    - If you're already providing a custom implementation of `Debug` for a struct,
      there's no need to include `#[derive(Debug)]` because it would override your
      custom implementation. Therefore, in such cases, it's best to either rely on
      the manual implementation for complete control over the debug representation or
      use `#[derive(Debug)]` if the default debug representation is sufficient for
      your needs.

    - In the context of the previous example, where we implemented custom formatting
      for the `Debug` trait, including `#[derive(Debug)]` would indeed be redundant
      and can be omitted. This ensures clarity and avoids potential confusion about
      which implementation takes precedence.
