



#![allow(clippy::let_unit_value)]
#![allow(clippy::no_effect)]
pub fn easy_rust_007_debuging_prinitng(){

    let my_number = {
        let second_number =8 ;
        second_number + 9;  // No semicolon, so the code block returns 8+9.

    };

    println!("My number is : {:#?}", my_number);

}


