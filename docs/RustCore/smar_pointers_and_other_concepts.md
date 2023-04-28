# Smart Pointers in Rust wiht Other concepts
Lets talk a bit about some `keywords` in Rust, to understand more about implementations and use cases

##### What is `dyn` keyword

In Rust, the &dyn keyword is used to define a reference to a type that
implements a particular trait, without specifying the concrete type of that
implementation.

- Here's a simple example to illustrate this:
```rust

trait Animal {
    fn speak(&self) -> &str;
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> &str {
        "woof"
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) -> &str {
        "meow"
    }
}

fn animal_speak(animal: &dyn Animal) {
    println!("{}", animal.speak());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    animal_speak(&dog);
    animal_speak(&cat);
}
```
- Here is another example: In this example, we define a `Shape` trait with an
  `area` method that returns a `f64` value representing the area of the shape.
  We implement this trait for two structs, Rectangular and Circle, each of
  which have their own implementation of the area method.

- We also define a `print_area` function that takes a reference to a value of
  any type that implements the Shape trait, using the `&dyn` Shape syntax to
  specify that the `reference` is to a `type` that implements the `Shape
  trait`, without specifying the `concrete type`.

- Finally, in the main function, we create values of type Rectangular and
  Circle, and pass references to these values to the print_area function.
  Because both Rectangular and Circle implement the `Shape` trait, the
  `print_area` function is able to call the `area` method on both of these
  values using the `&dyn` Shape reference.

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangular {
    width: f64,
    height: f64,
}

impl Shape for Rectangular {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangular {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 5.0 };

    print_area(&rectangle);
    print_area(&circle);
}

```

