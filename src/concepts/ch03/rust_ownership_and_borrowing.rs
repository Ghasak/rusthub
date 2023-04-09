


#[allow(unused_variables)]

pub fn rust_owner_ship_and_borrowing_concept(){

let stack_i8: i8 = 10;
let stack_f32: f32 = 20.;
let stack_bool: bool = true;
let stack_char: char = 'a';


let mut v :Vec<f32> = Vec::new();
    for i in 1..10{
        let k: f32 = i as f32;
        v.push(k)
    }
    for item in v.iter(){
        println!("{item}")
    }
}

