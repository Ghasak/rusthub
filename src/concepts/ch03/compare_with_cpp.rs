use colored::*;
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

pub fn allocate_on_heap() {
    // Allocate MyStruct on the heap
    let my_struct_boxed = Box::new(MyStruct::new(42));
    // Access data through the box
    println!("Data: {}", my_struct_boxed.data.to_string().yellow());
    // Box will be automatically dropped when it goes out of scope, freeing the memory
}
