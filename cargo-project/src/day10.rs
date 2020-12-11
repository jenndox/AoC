pub mod jolt {
    pub use crate::helper::helper_fns;

    pub fn convert(filename: &String) {
        let mut data = helper_fns::file_into_int_vec(filename);

        data.sort_by(|a,b| a.partial_cmp(b).unwrap());
        data.insert(0, 0);

        let combos = count_paths(&data, 0);

        println!("Combos: {}", combos);
    }

    fn count_paths(data: &Vec<i64>, index: i32) -> i32 {
        let mut paths;
        let mut options = 0;
        if index < (data.len() as i32 - 1) &&
                data[(index + 1) as usize] - data[index as usize] < 4 {
            options += count_paths(data, index + 1);
            if index < (data.len() as i32 - 2) &&
                    data[(index + 2) as usize] - data[index as usize] < 4 {
                options += count_paths(data, index + 2);
                if index < (data.len() as i32 - 3) &&
                        data[(index + 3) as usize] - data[index as usize] < 4 {
                    options += count_paths(data, index + 3);
                }
            }
            paths = options;

        } else {
            paths = 1;
        }
        paths
    }
 
    pub fn convert_1(filename: &String) {
        let mut data = helper_fns::file_into_int_vec(filename);
        let mut ones: i32 = 0;
        let mut threes: i32 = 0;
        let mut index: i32 = 0;

        data.sort_by(|a,b| a.partial_cmp(b).unwrap());

        while index < (data.len() - 1) as i32 {
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
