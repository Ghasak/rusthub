use crate::concepts::ch03::structs_traits_and_implementation::RandomInfo;
use itertools::Itertools;

pub fn generic_in_rust_concept() {
    #[derive(Debug)]
    enum RandomInfo<T> {
        OptionA(T),
        OptionB(T),
        OptionC,
    }

    let my_data: RandomInfo<f32> = RandomInfo::OptionA(100.23);
    let my_other_data: RandomInfo<f64> = RandomInfo::OptionB(400.23);
    let last_data: RandomInfo<f32> = RandomInfo::OptionC;
    let some_option: RandomInfo<&str> = RandomInfo::OptionA("This is just  a test ...");
    let some_vec: RandomInfo<Vec<String>> =
        RandomInfo::OptionA(vec!["G".to_owned(), "H".to_owned()]);

    println!("{my_data:#?}");
    println!("{some_vec:#?}");

    let input_x: [i32; 5] = [100, 200, 300, 400, 500];
    let output = experiment_function(&input_x);
    println!("{output:#?}");

    let our_set: &[char; 4] = &['A', 'B', 'C', 'D'];
    // use itertools::Itertools;
    // let permutations = our_set.iter().permutations(2);
    // for val in permutations {
    //     println!("{val:?}");
    // }
    let output = give_me_my_permutations(our_set, 3);
    match output {
        Ok(val) => println!("{val:?}"),
        Err(message) => (),
    }
}

#[allow(unused_assignments)]
pub fn experiment_function(param_a: &[i32; 5]) -> Vec<String> {
    let mut out_put_vec: Vec<String> = Vec::with_capacity(5);
    let mut output_message: String = String::new();
    for val in param_a {
        if val.to_string() == "100" {
            output_message = "This is $100".to_owned();
            out_put_vec.push(output_message);
        } else {
            output_message = format!("This is not acceptable {val}");
            out_put_vec.push(output_message);
        }
    }
    out_put_vec
}

/// Permutations and combination function
/// Input:
/// - `param_a`: `&[char]` -> Given the input of vector of characters.
pub fn give_me_my_permutations(param_a: &[char], k: usize) -> Result<Vec<Vec<&char>>, String> {
    let mut output_vec: Vec<Vec<&char>> = Vec::new();
    let permutations = param_a.iter().permutations(k);
    for val in permutations {
        println!("{:?}", val);
        output_vec.push(val)
    }
    Ok(output_vec)
}
