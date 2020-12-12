pub mod jolt {
    use std::collections::HashMap;
    pub use crate::helper::helper_fns;

    pub fn convert(filename: &String) {
        let mut index: i64 = 0;
        let mut four: u32 = 0;
        let mut three: u32 = 0;
        let mut two: u32 = 0;
        let base_seven: i32 = 7;
        let base_four: i32 = 4;
        let base_two: i32 = 2;
        let mut first_one = -1;

        let mut data = helper_fns::file_into_int_vec(filename);

        println!("Sorting");

        data.sort_by(|a,b| a.partial_cmp(b).unwrap());
        data.insert(0, 0);
        data.insert(data.len(), data[data.len() - 1] + 3);
        println!("Sorted");

        while index < (data.len() - 1) as i64 {
            let delta = data[(index + 1) as usize] - data[index as usize];

            match delta {
                1 => {
                    if first_one < 0 {
                        first_one = index;
                    }
                },
                3 => { 
                    let curr_streak = index - first_one;
                    match curr_streak {
                        4 => { four += 1; },
                        3 => { three += 1; },
                        2 => { two += 1; },
                        _ => { },
                    }
                    first_one = -1;
                },
                _ => { println!("WTF gap"); }
            }
            index += 1;
        }

        let answer: i64 = (base_seven.pow(four)) as i64 * (base_four.pow(three)) as i64 * (base_two.pow(two)) as i64;
        println!("four: {}, three {}, two: {}, solution: {}", four, three, two, answer);
    }

    fn _count_paths_interminable(data: &Vec<i64>, index: i64, mut visited: HashMap<i64, i64>) -> i64 {
        let paths: i64;
        let mut options = 0;
        let mut check_dist = 1;

        if visited.get(&index).is_some() {
            paths = *visited.get(&index).unwrap();
        } else {
            while index < (data.len() as i64 - check_dist) &&
                    data[(index + check_dist) as usize] - data[index as usize] < 4 &&
                    check_dist < 4 {
                options += _count_paths_interminable(data, index + check_dist, visited.clone());
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
