use crate::util::*;

pub fn day2() -> Result<(), String> {
    let lines = match util::read_file(String::from("src/days/day2/input.txt")) {
        None => vec![],
        Some(l) => l
    };

    let mut pos = 0;
    let mut depth = 0;

    for line in lines.iter() {
        let dir: Vec<&str> = line.split(" ").collect();

        match dir[0]  {
            "forward" => pos += dir[1].parse::<i32>().unwrap(),
            "up" => depth -= dir[1].parse::<i32>().unwrap(),
            "down" => depth += dir[1].parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("{}", pos * depth);

    return Ok(())
}

pub fn day2_pt2() -> Result<(), String> {
    let lines = match util::read_file(String::from("src/days/day2/input.txt")) {
        None => vec![],
        Some(l) => l
    };

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines.iter() {
        let dir: Vec<&str> = line.split(" ").collect();

        match dir[0]  {
            "forward" => {
                pos += dir[1].parse::<i32>().unwrap();
                depth += dir[1].parse::<i32>().unwrap() * aim;
            },
            "up" => aim -= dir[1].parse::<i32>().unwrap(),
            "down" => aim += dir[1].parse::<i32>().unwrap(),
            _ => {}
        }
    }

    println!("{}", pos * depth);
    return Ok(())
}