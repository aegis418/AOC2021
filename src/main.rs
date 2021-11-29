mod util;
mod days;

use std::io;
use crate::days::days::*;

fn main() {
    println!("AoC Runner");

    print!("Day to run: ");
    let mut day = String::new();
    io::stdin().read_line(&mut day)?;
    println!(format!("Running day {}...", day));
    run_day(Day::from::<i32>(day.parse::<i32>()));
}
