mod helper;
mod day1alt;
mod day2;
mod day3;
mod day4alt;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

pub use crate::day1alt::report;
pub use crate::day2::password;
pub use crate::day3::sled;
pub use crate::day4alt::passport;
pub use crate::day5::seats;
pub use crate::day6::customs;
pub use crate::day7::bags;
pub use crate::day8::device;
pub use crate::day9::xmas;
pub use crate::day10::jolt;
pub use crate::day11::lobby;
pub use crate::helper::helper_fns;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let filename = &args[2];

    println!("This is Day {} of advent", day);
    println!("Input file {}", filename);

    match day.parse().unwrap_or(1) {
        1 => report::report(filename, 3),
        201 => password::passwords_first(filename),
        2 => password::passwords(filename),
        3 => sled::toboggan(filename),
        4 => passport::passports(filename),
        5 => seats::seat_sort(filename),
        6 => customs::question(filename),
        7 => bags::shiny(filename),
        8 => device::instrs(filename),
        9 => xmas::cypher(filename),
        10 => jolt::convert(filename),
        11 => lobby::arrange(filename),
        // Handle the rest of cases
        _ => println!("Not an Advent Day we have done yet."),
    }
}

