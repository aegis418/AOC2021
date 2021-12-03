use crate::util::*;

pub fn day3() -> Result<(), String> {
    let bit_vec = match util::read_file(String::from("src/days/day3/input.txt")) {
        None => vec![],
        Some(f) => f
    };

    let mut gamma_vec: Vec<&str> = vec![];
    let mut epsilon_vec: Vec<&str> = vec![];

    for i in 0..12 {
        let mut num_one = 0;
        let mut num_zero = 0;
        for num in bit_vec.iter() {
            if num.as_bytes()[i] == "1".as_bytes()[0] {
                num_one += 1;
            } else {
                num_zero += 1;
            }
        }

        if num_one > num_zero {
            gamma_vec.push("1");
            epsilon_vec.push("0");
        } else {
            gamma_vec.push("0");
            epsilon_vec.push("1");
        }
    }

    let gamma_str = gamma_vec.concat();
    let epsilon_str = epsilon_vec.concat();

    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();

    println!("{}", gamma * epsilon);

    return Ok(())
}
// Returns pair with most common being the first item.
fn find_most_common_bit_in_pos(bit_list: &Vec<String>, pos: usize) -> (i32, i32) {
    let mut num_zero = 0;
    let mut  num_one = 0;
    for x in bit_list.iter() {
        if x.as_bytes()[pos] == "1".as_bytes()[0] {
            num_one += 1;
        } else {
            num_zero += 1;
        }
    };

    return if num_zero > num_one {
        (0, 1)
    } else if num_one > num_zero {
        (1, 0)
    } else {
        (-1, -1)
    }
}

pub fn day3_pt2() -> Result<(), String> {
    let bit_vec = match util::read_file(String::from("src/days/day3/input.txt")) {
        None => vec![],
        Some(f) => f
    };

    let mut oxygen_vec = bit_vec.clone();
    let mut co2_vec = bit_vec.clone();

    for pos in 0..12 as usize {
        let oxy_pair = find_most_common_bit_in_pos(&oxygen_vec, pos);
        // let co2_pair = find_most_common_bit_in_pos(&co2_vec, pos);
        //println!("O: {:?}", oxygen_vec);
        //println!("OP: {:?}", oxy_pair);

        if oxygen_vec.len() == 1 { break }
        match oxy_pair {
            (-1, -1) => {
                oxygen_vec.retain(|x| x.as_bytes()[pos] == 1.to_string().as_bytes()[0]);
            },
            (most@ 1, _) => {
                oxygen_vec.retain(|x| x.as_bytes()[pos] == most.to_string().as_bytes()[0]);
            },
            (most @ 0, _) => {
                oxygen_vec.retain(|x| x.as_bytes()[pos] == most.to_string().as_bytes()[0]);
            },
            (_, _) => {}
        };


    }

    for pos in 0..12 as usize {
        let co2_pair = find_most_common_bit_in_pos(&co2_vec, pos);
        if co2_vec.len() == 1 { break }
        match co2_pair {
            (-1, -1) => {
                co2_vec.retain(|x| x.as_bytes()[pos] == 0.to_string().as_bytes()[0]);
            },
            (_, least @ 0..=1) => {
                co2_vec.retain(|x| x.as_bytes()[pos] == least.to_string().as_bytes()[0]);
            },
            (_, _) => {}
        }
    }

    //println!("Final O: {:?}", oxygen_vec);
    //println!("Final C: {:?}", co2_vec);

    let oxygen_rating = u32::from_str_radix(oxygen_vec[0].as_str(), 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_vec[0].as_str(), 2).unwrap();

    println!("{}", oxygen_rating * co2_rating);

    return Ok(())
}