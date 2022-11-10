
#![allow(dead_code)]
pub fn experiment_sum_fn(){

    let mut s: String = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("maybe we can pass it .. ?");

    let s = s.trim().to_owned();

    println!("what is the value you input, is it {:#?}?", &s);
    println!("{}", super::common_collections::type_of(s));

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let nums = line
        .trim()
        .split(' ')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();
    let mut sum: f32 = 0.0;
    for num in nums {
        sum = sum + (num as f32);

        println!("{}", num);
    }
    println!("++++++++++++++++");
    println!("{}", sum);
}
