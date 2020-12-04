mod helper;
mod day1alt;
mod day2;
mod day3;

pub use crate::day1alt::report;
pub use crate::day2::password;
pub use crate::day3::sled;
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
        // Handle the rest of cases
        _ => println!("Not an Advent Day we have done yet."),
    }
}

