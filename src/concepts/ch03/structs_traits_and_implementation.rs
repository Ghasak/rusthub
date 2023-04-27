pub fn rust_structs_traits_and_implementation_fn() {
    #[derive(Debug)]
    pub struct RandomInfo {
        pub call_count: i64,
        pub some_bool: bool,
        pub some_int: i64,
    }
    impl RandomInfo {
        pub fn new(param_a: bool) -> Self {
            Self {
                call_count: 0,
                some_bool: !param_a,
                some_int: 8,
            }
        }
        pub fn is_smaller(&mut self, compare_to: i64) -> bool {
            self.call_count += 1;
            self.some_int < compare_to
        }
    }

    let compare_to_var = 100;
    let mut my_obj = RandomInfo::new(true);
    println!("{my_obj:#?}");
    let output = my_obj.is_smaller(compare_to_var);
    println!("{output:#?}");
    println!("{:#?}", my_obj.call_count);

    impl RandomInfo{
            pub fn is_larger(&self, compare_to: i64) -> bool{
                    self.some_int > compare_to
                }
        }
    let my_obj = RandomInfo::new(true);
    let output = my_obj.is_larger(compare_to_var);
    println!("{output:#?}");

}
