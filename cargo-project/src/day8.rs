pub mod device {
    pub use std::collections::HashMap;
    pub use crate::helper::helper_fns;

    pub fn instrs(filename: &String) {
        let data = helper_fns::file_into_vec(filename);
        let mut accumulator = 0;
        let mut accumulator_snap = 0;
        let mut loc: i32 = 0;
        let mut alternate: i32 = -1;
        let mut on_branch: bool = false;
        let mut tried: Vec<i32> = Vec::new();
        let mut found = false;

        let mut executed: HashMap<usize, bool> = HashMap::new();
        let mut executed_snap: HashMap<usize, bool> = HashMap::new();

        while !found { 
            let curr_instr = &data[loc as usize];
            let instr_parts: Vec<_> = curr_instr.split(' ').collect();
            
            if executed.get(&(loc as usize)) == Some(&true) {
                loc = alternate;
                println!("Reset alternate from {}", alternate);
                accumulator = accumulator_snap;
                println!("acc: {}", accumulator);
                on_branch = false;
                executed = executed_snap.clone();
            } else {
                executed.insert(loc as usize, true);

                let curr_value: i32 = instr_parts[1].trim().parse().expect("Cannot parse");
                println!("Curr value: {:?}, loc: {:?}", accumulator, loc);

                match instr_parts[0] {
                    "nop" => {
                        if on_branch == false && !tried.contains(&loc) {
                            // check if we can find with jump
                            println!("Trying: {}", loc);
                            tried.push(loc);
                            accumulator_snap = accumulator;
                            alternate = loc + 1;
                            loc += curr_value;
                            println!("next: {:?}, acc: {}", &data[loc as usize], accumulator);
                            on_branch = true;
                            executed_snap = executed.clone();
                        }
                        else {
                            loc += 1;
                        }
                    },
                    "acc" => {
                        accumulator += curr_value;
                        loc += 1;
                        println!("acc: {}", accumulator);
                    },
                    "jmp" => {
                        if on_branch == false && !tried.contains(&loc) {
                            // check if we can find with jump
                            println!("Trying: {}", loc);
                            tried.push(loc);
                            accumulator_snap = accumulator;
                            alternate = loc + curr_value;
                            loc += 1;
                            println!("next: {:?}, acc: {}", &data[loc as usize], accumulator);
                            on_branch = true;
                            executed_snap = executed.clone();
                        }
                        else {
                            loc += curr_value;
                        }
                    },
                    _ => {
                        println!("FOUND IT acc: {}", accumulator);
                        found = true; }
                }
            }
        }

        println!("Final accumulator: {}", accumulator);
    }
       
}
