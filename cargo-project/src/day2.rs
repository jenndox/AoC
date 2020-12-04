pub mod password {
    pub use crate::helper::helper_fns;

    pub fn passwords_first(filename: &String) {
        let mut success = 0;
        let data = helper_fns::file_into_vec(filename);

        for x in &data {
            let result: Vec<_> = x.split(' ').collect();
            let counts: Vec<_> = result[0].split('-').collect();
            let letterstr: Vec<_> = result[1].split(':').collect();
            let letter = letterstr[0];
            let password = result[2];
            let lower = counts[0].parse().unwrap_or(0);
            let upper = counts[1].parse().unwrap_or(0);

            let count: Vec<_> = password.matches(letter).collect();
            if count.len() >= lower && count.len() <= upper {
                success += 1; 
                println!("Successful password: {}", x);
            }
        }

        println!("Successful passwords: {}", success);
    }

    pub fn passwords(filename: &String) {
        let mut success = 0;
        let data = helper_fns::file_into_vec(filename);

        for x in &data {
            let result: Vec<_> = x.split(' ').collect();
            let counts: Vec<_> = result[0].split('-').collect();
            let letterstr: Vec<_> = result[1].split(':').collect();
            let letter: char = letterstr[0].chars().nth(0).unwrap_or(' ');
            let password = result[2];
            let lower = counts[0].parse().unwrap_or(0);
            let upper = counts[1].parse().unwrap_or(0);

            let passwordchars: Vec<_> = password.chars().collect();
            if (passwordchars[lower - 1] == letter || passwordchars[upper - 1] == letter) &&
                !(passwordchars[lower - 1] == letter && passwordchars[upper - 1] == letter) {
                success += 1; 
                println!("Successful password: {}", x);
            }
        }

        println!("Successful passwords: {}", success);
    }
}

