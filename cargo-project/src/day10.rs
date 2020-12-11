pub mod jolt {
    use std::collections::HashMap;
    pub use crate::helper::helper_fns;

    pub fn convert(filename: &String) {
        let mut data = helper_fns::file_into_int_vec(filename);
        let mut visited: HashMap<i64, i64> = HashMap::new();

        println!("Sorting");

        data.sort_by(|a,b| a.partial_cmp(b).unwrap());
        data.insert(0, 0);
        println!("Sorted");

        let combos = count_paths(&data, 0, visited);

        println!("Combos: {}", combos);
    }

    fn count_paths(data: &Vec<i64>, index: i64, mut visited: HashMap<i64, i64>) -> i64 {
        let paths: i64;
        let mut options = 0;
        let mut check_dist = 1;

        if visited.get(&index).is_some() {
            paths = *visited.get(&index).unwrap();
        } else {
            while index < (data.len() as i64 - check_dist) &&
                    data[(index + check_dist) as usize] - data[index as usize] < 4 &&
                    check_dist < 4 {
                options += count_paths(data, index + check_dist, visited.clone());
                check_dist +=1;
            }
            if check_dist == 1 { options = 1; }
            paths = options;
            visited.insert(index, paths);
        }
        paths
    }
 
    pub fn convert_1(filename: &String) {
        let mut data = helper_fns::file_into_int_vec(filename);
        let mut ones: i64 = 0;
        let mut threes: i64 = 0;
        let mut index: i64 = 0;

        data.sort_by(|a,b| a.partial_cmp(b).unwrap());

        while index < (data.len() - 1) as i64 {
            let delta = data[(index + 1) as usize] - data[index as usize];

            match delta {
                1 => { ones += 1; },
                2 => { },
                3 => { threes += 1; },
                _ => { println!("WTF"); break; }
            }
            index += 1;
        }

        ones += 1;
        threes += 1;
        println!("Ones: {}, threes {}, multiplied: {}", ones, threes, ones * threes);
    } 
}
