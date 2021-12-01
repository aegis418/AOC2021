use crate::util::*;

pub fn day_1() -> Result<(), String> {
    let string_vec = match util::read_file(String::from("src/days/day1/input.txt")) {
        None => vec![],
        Some(f) => f
    };

    let num_vec: Vec<u32> = string_vec.iter().map(|val| val.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut ans: u32 = 0;
    for i in 1..(num_vec.len() - 1) {
        if num_vec[i+1] > num_vec[i] {
            ans += 1;
        }
    }

    println!("{}", ans);

    return Ok(())
}

pub fn day_1_pt2() -> Result<(), String> {
    let string_vec = match util::read_file(String::from("src/days/day1/input.txt")) {
        None => vec![],
        Some(f) => f
    };

    let num_vec: Vec<u32> = string_vec.iter().map(|val| val.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut ans = 0;

    let mut i = 0;
    let mut j = 1;
    let mut k = 2;
    let mut last_sum = u32::MAX;

    while k < num_vec.len() {
        let cur_sum = num_vec[i] + num_vec[j] + num_vec[k];
        if cur_sum > last_sum {
            ans += 1;
        }
        i += 1;
        j += 1;
        k += 1;
        last_sum = cur_sum;
    }

    println!("{}", ans);
    return Ok(())
}