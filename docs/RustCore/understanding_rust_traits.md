# Understanding Rust Traits

Finally, I can say that I have understood the concept behind the `derive` which
gives us a way to attach any trait (e.g., from the standard library of Rust,
from other packages or custom designed). Here, I will show the `custom designed
trait` that I used to attached to a `struct` that I use to carry the
information of three fields.

- You need to review and read the implemation of the `rust trait` at `../../src/concepts/ch03/understanding_traits_concept.rs`.


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

