pub mod report {
    pub use crate::helper::helper_fns;

    pub fn report(filename: &String) {
        let data: Vec<i64> = helper_fns::file_into_int_vec(filename);

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
}

