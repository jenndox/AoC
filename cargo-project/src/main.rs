use std::{
    fs::File,
    io::{prelude::*, BufReader},
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let filename = &args[2];

    println!("This is Day {} of advent", day);
    println!("Input file {}", filename);

    match day.parse().unwrap_or(1) {
        1 => report(filename),
        // Handle the rest of cases
        _ => println!("Not an Advent Day we have done yet."),
    }

}

fn report(filename: &String) {

    let contents = File::open(filename)
        .expect("Something went wrong reading the file");
    let buf = BufReader::new(contents);
    let data: Vec<_> = buf.lines()
        .map(|l| l.expect("Could not parse line").parse().unwrap_or(0))
        .collect();

    for x in &data {
        for y in &data {
            for z in &data {
                if (*x + *y + *z) == 2020 {
                    println!("Found the TARGET! {} {} {}", x, y, z);
                    println!("Answer is {}", x * y * z);
                    return;
                }
            }
        }
    }

}
