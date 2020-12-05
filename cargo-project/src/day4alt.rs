pub mod passport {
    pub use crate::helper::helper_fns;
    use std::i64;

    pub fn passports(filename: &String) {
        let mut valid = 0;
        let data = helper_fns::file_into_vec(filename);
        let mut byr = 0;
        let mut iyr = 0;
        let mut eyr = 0;
        let mut hgt = 0;
        let mut hcl = 0;
        let mut ecl = 0;
        let mut pid = 0;

        for x in &data {
            let result: Vec<_> = x.split(' ').collect();

            if result.len() > 0 && result[0].chars().count() > 0 {
                for segment in result {
                    let parts: Vec<_> = segment.split(':').collect();
                    let prefix = parts[0];
                    let suffix = parts[1];
                    match prefix {
                        "byr" => {
                             if suffix.chars().count() == 4 && suffix.parse().unwrap_or(0) >= 1920 && suffix.parse().unwrap_or(0) <= 2002 {
                                 byr = 1;
                             }
                         },
                        "iyr" => {
                             if suffix.chars().count() == 4 && suffix.parse().unwrap_or(0) >= 2010 && suffix.parse().unwrap_or(0) <= 2020 {
                                 iyr = 1;
                             }
                         },
                        "eyr" => {
                             if suffix.chars().count() == 4 && suffix.parse().unwrap_or(0) >= 2020 && suffix.parse().unwrap_or(0) <= 2030 {
                                 eyr = 1;
                             }
                         },
                        "hgt" => {
                             let metric: Vec<_> = suffix.split('c').collect();
                             let english: Vec<_> = suffix.split('i').collect();
                             if metric.len() > 1 && metric[1] == "m" && metric[0].parse().unwrap_or(0) >= 150 && metric[0].parse().unwrap_or(0) <= 193 {
                                 hgt = 1;
                             } else {
                                 if english.len() > 1 && english[1] == "n" && english[0].parse().unwrap_or(0) >= 59 && english[0].parse().unwrap_or(0) <= 76 {
                                     hgt = 1;
                                 } 
                             }
                         },
                        "hcl" => {
                             let value: Vec<_> = suffix.split('#').collect();
                             if suffix.chars().count() == 7 && value.len() > 1 && i64::from_str_radix(value[1], 16).is_ok() {
                                 hcl = 1;
                             }
                         },
                        "ecl" => {
                             if suffix == "amb" || suffix == "blu" || suffix == "brn" || suffix == "gry" || suffix == "grn" || suffix == "hzl" || suffix == "oth" {
                                 ecl = 1;
                             }
                         },
                        "pid" => {
                             if suffix.chars().count() == 9 && suffix.parse().unwrap_or(-1) >= 0 {
                                 pid = 1;
                             }
                         },
                        _ => println!("Other field: {}", prefix),
                    }
                }
            }
            else {
                if byr == 1 && iyr == 1 && eyr == 1 && hgt == 1 && hcl == 1 && ecl == 1 && pid == 1 {
                    valid += 1;
                    println!("End of valid passport.");
                }
                else {
                    println!("End of INvalid passport.");
                }
                byr = 0;
                iyr = 0;
                eyr = 0;
                hgt = 0;
                hcl = 0;
                ecl = 0;
                pid = 0;
            }
        }

        println!("Successful passport validations: {}", valid);
    }
}
