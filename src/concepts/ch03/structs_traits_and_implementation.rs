use rand::Rng;

#[derive(Debug)]
pub struct RandomInfo {
    some_int: i32,
    some_vect: Vec<String>,
}

impl Default for RandomInfo {
    fn default() -> Self {
        Self {
            some_int: 10,
            some_vect: vec!["AAA".to_string(), "BBB".to_string()],
        }
    }
}
impl RandomInfo {
    fn new(param_a: i32, param_b: &Vec<String>) -> Self {
        Self {
            some_int: param_a,
            some_vect: param_b.to_owned(),
        }
    }

    fn show_information(&self, verbose: bool) -> String {
        let mut object_report_string = String::new();
        object_report_string.push_str(&format!(
            "Following object with attributes -> \nsome_int:{:#?} and\nsome_vec:{:#?}",
            self.some_int, self.some_vect
        ));
        object_report_string
    }

    fn computional_implementation(&mut self, value: i32){
        for i in 0..value{
            self.some_int = self.some_int + i;
            println!("[+] -> getting the value => {:#?} from i value => {:.20} ", self.some_int, i)
        }
    }
}

/// This is a demonstration function
pub fn rust_structs_traits_and_implementation_fn() {
    let some_vec: Vec<String> = vec!["A".to_owned(), "B".to_owned()];
    let my_obj = RandomInfo::default();
    println!("{:#?}", my_obj);
    let mut another_obj = RandomInfo::new(3, &some_vec);
    println!("{another_obj:#?}");
    println!("{}", another_obj.show_information(true));
    println!("{:#?}", another_obj.computional_implementation(10));
}


