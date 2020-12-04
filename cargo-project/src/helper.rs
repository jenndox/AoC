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

    pub fn file_into_array(filename: &String) -> [[char; 31]; 323] {
        const M: usize = 323;
        const N: usize = 31;
        let contents = File::open(filename)
            .expect("Something went wrong reading the file");
        let buf = BufReader::new(contents);
        let data: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        let mut results: [[char; N]; M] = [[' '; N]; M];
        let mut i = 0;
        let mut j;
        for line in &data {
            j = 0;
            for col in line.chars() {
                results[i][j] = col;
                j += 1;
                if i >= M || j >= N {
                    break;
                }
            }
            i += 1;
        }
        results
    }
}
