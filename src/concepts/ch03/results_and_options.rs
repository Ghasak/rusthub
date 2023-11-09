use std::num::ParseIntError;
use std::process;

pub fn results_and_option() {
    let examine_vector: Vec<String> = vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("abc"),
    ];
    for item in examine_vector {
        println!("We have element -> {item:#?}");
        let output = parse_int(&item);
        if let Ok(val) = output {
            println!("output is -> {val:#?}");
        } else {
            println!("ERROR:: In parsing at item : {item:#?}")
        }
    }
    let output = git_revision_hash();
    if let Some(val) = output {
        let lines: Vec<&str> = val.split('\n').collect();
        for line in lines {
            println!("output -> {line:#?}");
        }
    }
}

pub fn parse_int(string_input: &str) -> Result<i32, ParseIntError> {
    let n = string_input.parse()?;
    Ok(n)
}

// Converting Results to Option
pub fn parse_int_option(string_input: &str) -> Result<i32, String> {
    let output = string_input.parse().ok();
    match output {
        Some(val) => Ok(val),
        None => Err("::Error Occured:: ".to_string()),
    }
}

fn git_revision_hash() -> Option<String> {
    let result = process::Command::new("git")
        //.args(&["rev-parse", "--short=10", "HEAD"])
        .args(["logs", "-10"])
        .output();
    result.ok().and_then(|output| {
        let v: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}

#[derive(Debug, Clone)]
pub struct MyInfo {
    pub user_name: String,
    pub last_name: String,
    pub user_status: bool,
    pub user_id: u32,
    pub user_email: String,
}
impl MyInfo {
    pub fn new(
        user_name: String,
        last_name: String,
        user_status: bool,
        user_id: u32,
        user_email: String,
    ) -> Self {
        Self {
            user_name,
            last_name,
            user_status,
            user_id,
            user_email,
        }
    }
}


pub fn testing_my_info(my_info: MyInfo) -> Option<String> {
    let output_message =
        format!("Output of the current function is not Null -> Awesome Perforamnce ... ");
    if my_info.user_status == true {
        Some(output_message)
} else {
        None
    }

}
