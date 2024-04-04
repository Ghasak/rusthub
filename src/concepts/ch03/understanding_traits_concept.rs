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

