pub mod xmas {
    pub use crate::helper::helper_fns;

    pub fn cypher(filename: &String) {
        let data = helper_fns::file_into_vec(filename);
        let mut rolling: i32 = 0;
        let mismatch: i32 = 26134589;
        let mut index = 0;
        let mut init_index = 0;
        let mut found_match = false;

        while !found_match {
            let curr_number: i32 = data[index].parse().unwrap_or(-1);

            rolling += curr_number;

            if rolling == mismatch {
                let mut smallest = data[init_index].parse().unwrap_or(-1);
                let mut largest = smallest;

                for rolling_val in &data[init_index..index] {
                    let this_val = rolling_val.parse().unwrap();
                    if this_val > largest { largest = this_val; }
                    if this_val < smallest { smallest = this_val; }
                }
                println!("Smallest: {}, largest: {}, added: {}", smallest, largest, (smallest + largest));
                found_match = true;
            } else if rolling > mismatch {
                init_index += 1;
                index = init_index;
                rolling = 0;
            } else {
                index += 1;
            }
        }
    }

    pub fn cypher_1(filename: &String) {
        let data = helper_fns::file_into_vec(filename);
        let mut rolling: Vec<i32> = Vec::new();
        let mut mismatch: i32 = -1;
        let mut index = 0;
        let mut lower: Vec<i32> = Vec::new();
        let mut upper: Vec<i32> = Vec::new();
        let mut found_match = false;

        while mismatch < 0 {
            let curr_number: i32 = data[index].parse().unwrap_or(-1);
            if index < 26 {
                println!("First 5 numbers: {:?}, curr_number: {}", rolling, curr_number);
                rolling.push(curr_number);
            } else {
                println!("{:?}", rolling);
                for num_check in &rolling {
                    if num_check <= &(&curr_number/2) { lower.push(*num_check); }
                    if num_check >= &(&curr_number/2) { upper.push(*num_check); }
                }

                for potential_lower in &lower {
                    if upper.contains(&(curr_number - potential_lower)) {
                        found_match = true;
                    }
                }

                    println!("lower: {:?}, Current number: {}, all upper: {:?}", lower, curr_number, upper);
                if !found_match {
                    mismatch = curr_number;
                }

                found_match = false;
                lower.clear();
                upper.clear();

                rolling.push(curr_number);
                rolling.remove(0);
            }
            index += 1;
        }

        println!("found mismatch: {}", mismatch);
    }
       
}
