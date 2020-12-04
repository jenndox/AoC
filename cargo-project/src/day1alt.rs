pub mod report {
    pub use crate::helper::helper_fns;
    pub use std::collections::HashMap;

    pub fn report(filename: &String, target_count: i32) {
        let data: Vec<i64> = helper_fns::file_into_int_vec(filename);

        let mut scanned_nums: HashMap<i64, Vec<Vec<i64>>> = HashMap::new();
        let mut matches: Vec<Vec<i64>> = Vec::new();

        for x in &data {
            if !scanned_nums.contains_key(x) {
                scanned_nums.insert(*x, vec![vec![*x]]);
            }
            if scanned_nums.contains_key(&(2020 - *x)) {
                // Found a match
                let result_set = scanned_nums.get(&(2020 - *x)).unwrap().clone();
                for result in result_set {
                    let mut res_copy = result.clone();
                    res_copy.push(*x);
                    matches.push(res_copy.to_vec());
                }
            }

            // See if any previously scanned combinations work with x.
            let mut partials: HashMap<i64, Vec<Vec<i64>>> = HashMap::new();
            for (k, v) in scanned_nums.iter() {
                if *k + *x < 2020 {
                    // include this for future reference
                    let partial_set = v.clone();
                    for partial in partial_set {
                        let mut partial_copy = partial.clone();
                        partial_copy.push(*x);
                        partials.insert(*k + *x, [partial_copy.to_vec()].to_vec());
                    }
                }
            }
            scanned_nums.extend(partials);
        }

        // Found all the options, filter to the ones with the target count
        let results: Vec<Vec<i64>> = matches.into_iter().filter(|v| v.len() == target_count as usize).collect();
        println!("matches: {}", results.len());

        for v in results.iter() {
            println!("Found a match! {:#?}", v);

            let mut multiplied = 1;
            for i in v.iter() {
                multiplied *= *i;
            }
            println!("Your solution! {:#?}", multiplied);
        }
    }
}

