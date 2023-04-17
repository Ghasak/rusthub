use std::io;
/// # Demonstration Function
/// ## Function Highlights
/// ### Input
/// The function require some inputs for demonstration.
pub fn life_time_concept_fn() {
    let x: i32 = 10;

    for i in 0..10 {
        println!("This is the right way to make things working ->  {i}");
    }

    let k: Vec<&str> = vec!["Jackson", "Michal", "Right"];
    for item in k {
        println!("{item}")
    }

    let k = working_style(2.0, 1.0);
    println!("{:#?}", k.unwrap());
    //     match &k {
    //         Ok(result) => println!("The value of k = {:#?}",k),
    //         Err(message) => println!("{message}")
    //     }
    //     println!("{:#?}", &k);
    // }
    //
    let result: Option<String> = advanced_function_testing("Wow1", "Wow2");
    println!("{result:#?}")
}

fn working_style(mut a: f64, mut b: f64) -> Result<f64, String> {
    a = a + 0.001;
    b = b + 0.001;
    let c: f64 = a + b;
    if c < 10.0 {
        Ok(c)
    } else {
        Err("This is not acceptable, c > 10 ".to_owned())
    }
}
///A silly funciton for testing
fn advanced_function_testing<'a>(param_a: &'a str, param_b: &'a str) -> Option<String> {
    let param_c: String = format!("{},{}", param_a, param_b);
    if param_c == "Wow1,Wow2" {
        None
    } else {
        Some(param_c)
    }
}
// fn advanced_function_testing<'a>(param_a: &'a str, param_b: &'a str) -> Option<&'a str> {
//     let param_c: Box<String> = Box::new(format!("{},{}", param_a, param_b));
//     Some(&*param_c)
// }
