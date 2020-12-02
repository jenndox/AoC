
pub mod helper_fns {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    pub fn file_into_vec(filename: &String) -> Vec<String> {
        let contents = File::open(filename)
            .expect("Something went wrong reading the file");
        let buf = BufReader::new(contents);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    pub fn file_into_int_vec(filename: &String) -> Vec<i64> {
        let contents = File::open(filename)
            .expect("Something went wrong reading the file");
        let buf = BufReader::new(contents);
        buf.lines()
            .map(|l| l.expect("Could not parse line").parse().unwrap_or(0))
            .collect()
    }
}
