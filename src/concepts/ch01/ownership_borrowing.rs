#![allow(dead_code)]

pub fn about_owner_ship_concepts() {
    let x = 12;
    let y = 14;

    // we are calling form our current module a function called squre.
    let z = super::ownership_borrowing::square(x);

    println!("value of x = {}, y = {} and z = {}", x, y, z);
    // If you want to call a function from parent directory.

    crate::concepts::create_text();

    println!("************************************");

    let s1: String = String::from("Hello World !");
    //let s2 = s1;
    let s2 = &s1;

    //println!("{}", s2.to_owned());
    //println!("{}", s2.clone());
    println!("{}{:p} <=> {:p}", s2, &s2, &s1);

    println!("************************************");
    super::ownership_borrowing::func1(&s1);
}

pub fn square(a: u32) -> u32 {
    a * a
}

pub fn func1(s: &String) {
    println!("From func: {}", s);
}
