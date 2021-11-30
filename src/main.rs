mod util;
mod days;

use std::io;
use std::io::Write;
use crate::days::days::*;

fn main() {
    println!("AoC Runner");

    print!("Day to run: ");
    io::stdout().flush();
    let mut day = String::new();
    io::stdin().read_line(&mut day).unwrap();
    println!("{}", format!("Running day {}...", day));
    match run_day(Day::from(day.parse::<i32>().unwrap())) {
        Ok(_) => (),
        Err(e) => (),
    };
}
