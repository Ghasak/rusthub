# About Struct

## Rules

- For `static` methods associated with a `strcut` you use the method spearater
  to call them using `::`. `static` method is known that the function signature has no `&self`.

- While for for `instance` methods you use the
  conventional way which is the `dot` notation. `instance` method has a `&self` in the signature.

## How to create a constrcutor

```rust
    #[derive(Debug)]
    struct Rectangle  {
        width: u32,
        height: u32
    }

    impl Rectangle{
        // this can be considered as a constructor for the class Rectangle, with the method new,
        fn __init__(width: u32, height: u32) -> Rectangle{
            Rectangle {
            width, height
        }
        }
        fn get_width(&self) -> u32 {
            self.width
        }

        fn get_height(&self) -> u32 {
            self.height
        }

        fn area(&self) -> u32{
            self.width * self.height
        }
    }


    let rect1 = Rectangle{
        width : 30,
        height: 50,
    };

    println!("The area of the Rectangle is { } square pixels ...",
        rect1.area()

    );
    let new_rect = Rectangle::__init__(100, 200);
    println!("Lets constrct a new rectangular using the init method that we created => {:#?}", new_rect)


```

- The keyword `Self` -> `Rectangle` thus, in the `static` method, we can use it
  to return `Self` instead the name `Rectangle`, **Notice** that the `Self` is
  `camel_case` with capital `S`.

```rust
    fn __init__(width: u32, height: u32) -> Self{
         Self{
        width, height
    }
    }

```

- You can sepeated `imp` with two or more blocks which will be helpful later
  when we use it in `generic` and `traits`.
