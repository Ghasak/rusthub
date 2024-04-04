# Allocating on HEAP

Here, we will draw our conclusion over the `Rust` and the `C++` in term of
allocating on heap.

## Allocating on heap in Rust

Here's an example in Rust allocating a struct on the heap and then the
equivalent example in C++. I'll provide explanations along with a comparison
table at the end.

### Rust Example:

```rust
// Define a simple struct
struct MyStruct {
    data: i32,
}

impl MyStruct {
    // Constructor function for MyStruct
    fn new(data: i32) -> MyStruct {
        MyStruct { data }
    }
}

fn main() {
    // Allocate MyStruct on the heap
    let my_struct_boxed = Box::new(MyStruct::new(42));

    // Access data through the box
    println!("Data: {}", my_struct_boxed.data);
    // Box will be automatically dropped when it goes out of scope, freeing the memory
}
```

## Allocating on heap in C++

Same example but this time we use `raw` pointer and allow to use this concept.

### C++ Example:

```cpp
#include <iostream>

// Define a simple struct
struct MyStruct {
    int data;
    // Constructor
    MyStruct(int d) : data(d) {}
};

int main() {
    // Allocate MyStruct on the heap
    MyStruct* my_struct_dynamic = new MyStruct(42);

    // Access data through the pointer
    std::cout << "Data: " << my_struct_dynamic->data << std::endl;

    // Clean up memory
    delete my_struct_dynamic;
}
```

### Comparison Table:

| Feature           | Rust                                         | C++                                                                        |
| ----------------- | -------------------------------------------- | -------------------------------------------------------------------------- |
| Language Syntax   | Modern, concise syntax                       | Syntax varies, but generally more verbose                                  |
| Memory Management | Handled through ownership and borrowing      | Manual memory management via `new` and `delete`                            |
| Memory Safety     | Strongly enforced                            | Lacks strict enforcement, prone to leaks/dangling pointers                 |
| Error Handling    | Through Result and Option types              | Via exceptions and/or return codes                                         |
| Standard Library  | Rich standard library with powerful features | Standard library provides essential utilities but not as extensive as Rust |
| Compilation       | Compiled to machine code                     | Compiled to machine code                                                   |
| Performance       | Generally on par with C/C++                  | Generally on par with Rust                                                 |
| Community Support | Growing community and ecosystem              | Mature community and extensive ecosystem                                   |

Both languages provide mechanisms for heap allocation and clean-up, but Rust's
approach with ownership and borrowing leads to safer code with less risk of
memory-related bugs. C++, on the other hand, offers more flexibility but
requires developers to be more vigilant about memory management.

