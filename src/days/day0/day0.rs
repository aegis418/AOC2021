use crate::util::*;

pub fn day_0() -> Result<(), String> {
    let file_arr: Vec<String> = match util::read_file(String::from("src/days/day0/test.txt")) {
        None => vec![],
        Some(arr) => arr
    };
    println!("{:?}", file_arr);
    Ok(())
}