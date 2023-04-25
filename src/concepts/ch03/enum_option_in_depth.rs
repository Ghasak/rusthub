

pub fn option_enum_concept_fn() {
    let my_vect: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

    if (my_vect.len() % 2) == 0 {
        println!("Yes, the vector is even ...")
    } else {
        println!("No, the vector is odd ..")
    }

    let result: Result<_, String> = match my_vect.len() % 2 {
        0 => Ok(()),
        _ => Err(String::from("The length of the vector is odd ...")),
    };
    let my_vect_f64: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
    let output: Result<(Vec<f64>, Vec<f64>), String> = extract_two_vectors_from_one(&my_vect_f64);
}

fn extract_two_vectors_from_one(my_vect: &Vec<f64>) -> Result<(Vec<f64>, Vec<f64>), String> {
    if my_vect.len() % 2 == 0 {
        println!("Yes, the vector is even...");
        let d: usize = my_vect.len() / 2;
        let half_vec1: Vec<f64> = my_vect[0..d].to_vec();
        let second_half_vec2: Vec<f64> = my_vect[d..my_vect.len()].to_vec();
        return Ok((half_vec1, second_half_vec2));
    }
    Err("The vector is not even.".to_string())
}
