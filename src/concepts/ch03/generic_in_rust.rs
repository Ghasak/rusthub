pub fn generic_in_rust_concept() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let point_1 = Point {
        x: 1012.23,
        y: 1121.44,
    };

    let point_2 = Point { x: 12, y: 10 };

    let point_2 = Point { x: 12.0, y: 10 };

    let point_2 = Point { x: 12, y: 10.12 };
    // Accept float

    println!("{point_1:#?}");
    // Accept Int
    println!("{point_2:#?}");

    for i in 0..10 {
        println!("The value of i -> {i}")

    }
}
