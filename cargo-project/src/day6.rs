pub mod customs {
    pub use crate::helper::helper_fns;

    pub fn question(filename: &String) {
        let data = helper_fns::file_into_vec(filename);

        let mut total_ct = 0;
        let mut have_group = 0;
        let mut answers: Vec<char> = Vec::new();

        for x in &data {
            let result: Vec<char> = x.chars().collect();

            if result.len() > 0 {
                if have_group == 0 {
                    for letter in result {
                        // Establish the super set we care about
                        answers.push(letter);
                    }
                    have_group = 1;
                }
                else {
                    answers.retain(|&x| result.contains(&x));
                }
            }
            else {
                println!("Group had: {:?}", answers);
                println!("Group had: {}", answers.len());
                total_ct += answers.len();
                // reset HashMap
                answers.clear();
                have_group = 0;
            }
        }

        println!("Total yeses: {}", total_ct);
    }
}
