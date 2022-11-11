/// a struct or stucutre, is a custom data type that lets you package together and name multiple related values that maek up a meaningful group.
pub fn using_structs_to_structure_related_data() {
    #[derive(Debug)]
    struct my_new_struct {
        name: String,
        address: String,
        telephone: String,
    }

    let mut v: Vec<my_new_struct> = Vec::new();

    let a = my_new_struct {
        name: "Michael Jackson".to_string(),
        address: "New York".to_string(),
        telephone: "090-2323-121".to_string(),
    };

    let b = my_new_struct {
        name: "Jason Momoa".to_string(),
        address: "Hawaii".to_string(),
        telephone: "090-12121-121".to_string(),
    };

    v.push(a);
    v.push(b);
    //println!("Current vector is = {:#?}", v);
    println!("==============================================",);
    println!("   using struct to structure related data     ");
    println!("==============================================",);

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("current user => {:#?}", user1);
    println!("------> before");
    println!("Accessing elements of user\nuser name: {}\nemail address: {}\nactive: {}\nsign in count: {}", user1.username, user1.email,user1.active,user1.sign_in_count);

    user1.username = String::from("Jackson Michael");

    println!("------> after");
    println!("Accessing elements of user\nuser name: {}\nemail address: {}\nactive: {}\nsign in count: {}", user1.username, user1.email,user1.active,user1.sign_in_count);

    println!("==============================================",);
    println!("       Method syntax                           ");
    println!("==============================================",);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // this can be considered as a constructor for the class Rectangle, with the method new,
        fn __init__(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        fn get_width(&self) -> u32 {
            self.width
        }

        fn get_height(&self) -> u32 {
            self.height
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the Rectangle is { } square pixels ...",
        rect1.area()
    );
    let new_rect = Rectangle::__init__(100, 200);
    println!(
        "Lets constrct a new rectangular using the init method that we created => {:#?}",
        new_rect
    );
    println!(
        "Accessing indivdual methods for new_rect width: {:#?} and hight: {:#?}",
        new_rect.get_width(),
        new_rect.get_height()
    );

    println!("==============================================",);
    println!("       Example Transfromed for Python          ");
    println!("==============================================",);

    #[derive(Debug)]
    /// An Employee struct is represented here ...
    /// Has five Arguments to be modified

    struct Employee {
        first_name: String,
        last_name: String,
        email_address: String,
        age: u32,
        salary: f32,
        id: u8,
    }

    impl Employee {
        /// Return an empolyee with the name given them, with other attributes ...
        /// Using constructor method called `new`
        ///  # Arguments
        ///  * `first name`      - A String allocated on the heap for storing the fist name
        ///  * `last name`       - A String allocated on the heap for storing the last name
        ///  * `email address`   - A String allocated on the heap for storing the email address
        ///  * `age`             - A String allocated on the heap for storing the fist name age
        ///  * `salary`          - A String allocated on the heap for storing the fist name salary

        const VARX: u32 = 0;

        fn new(
            first_name: String,
            last_name: String,
            email_address: String,
            age: u32,
            salary: f32,
            id: u8,
        ) -> Self {
            Self {
                first_name,
                last_name,
                email_address,
                age,
                salary,
                id,
            }
        }

        /// getting the employee object using the __str__method()
        fn __str__(&self) -> String {
            let __str__ = format!(
                "employeeID: {} ->first name: {}-> last name: {} -> email address: {} -> age: {} -> salary{}",
                Employee::VARX+1, self.first_name, self.last_name, self.email_address, self.age, self.salary
            );
            __str__
        }
    }

    let emp1 = Employee::new(
        "Jimmy".to_owned(),
        "Carter".to_owned(),
        "Jimmy_Cartor@gmail.com".to_owned(),
        23,
        122122.234,
        1,
    );

    println!(
        "How about checking our first employee\n: {:#?}\n",
        emp1.__str__()
    );

    let emp2 = Employee::new(
        "TJ".to_owned(),
        "Deverise".to_owned(),
        "TJ_Deverise@gmail.com".to_owned(),
        27,
        122122.234,
        2,
    );

    println!(
        "How about checking our first employee\n: {:#?}\n",
        emp1.__str__()
    );
}
