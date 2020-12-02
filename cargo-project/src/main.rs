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
        2 => passwords(filename),
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

fn passwordsFirst(filename: &String) {
    let mut success = 0;

    let contents = File::open(filename)
        .expect("Something went wrong reading the file");
    let buf = BufReader::new(contents);
    let data: Vec<_> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    for x in &data {
        let result: Vec<_> = x.split(' ').collect();
        let counts: Vec<_> = result[0].split('-').collect();
        let letterStr: Vec<_> = result[1].split(':').collect();
        let letter = letterStr[0];
        let password = result[2];
        let lower = counts[0].parse().unwrap_or(0);
        let upper = counts[1].parse().unwrap_or(0);

        let countForLetter: Vec<_> = password.matches(letter).collect();
        if countForLetter.len() >= lower && countForLetter.len() <= upper {
            success += 1; 
            println!("Successful password: {}", x);
        }
    }

    println!("Successful passwords: {}", success);
}

fn passwords(filename: &String) {
    let mut success = 0;

    let contents = File::open(filename)
        .expect("Something went wrong reading the file");
    let buf = BufReader::new(contents);
    let data: Vec<_> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    for x in &data {
        let result: Vec<_> = x.split(' ').collect();
        let counts: Vec<_> = result[0].split('-').collect();
        let letterStr: Vec<_> = result[1].split(':').collect();
        let letter: char = letterStr[0].chars().nth(0).unwrap_or(' ');
        let password = result[2];
        let lower = counts[0].parse().unwrap_or(0);
        let upper = counts[1].parse().unwrap_or(0);

        let passwordChars: Vec<_> = password.chars().collect();
        if (passwordChars[lower - 1] == letter || passwordChars[upper - 1] == letter) &&
            !(passwordChars[lower - 1] == letter && passwordChars[upper - 1] == letter) {
            success += 1; 
            println!("Successful password: {}", x);
        }
    }

    println!("Successful passwords: {}", success);
}
