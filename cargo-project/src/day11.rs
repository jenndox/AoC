pub mod lobby {
    pub use crate::helper::helper_fns;

    pub fn arrange(filename: &String) {
        let mut data: Vec<String> = helper_fns::file_into_vec(filename);
        let mut changes = true; // We know the first one will always have some changes.
        let mut new_data: Vec<String> = Vec::new();
        let mut tries = 0;
        let mut x = 0;
        let mut y = 0;

        while changes && tries < 3 {
            changes = false;

            for row in &data {
                let mut new_row = "".to_string();
                for seat in row.chars().collect::<Vec<char>>() {
                    match seat {
                        'L' => { 
                            if count_full_prox(x, y, &data) == 0 {
                                new_row.push('#');
                                changes = true;
                            } else {
                                new_row.push('L');
                            }
                        },
                        '#' => { 
                            if count_full_prox(x, y, &data) >= 5 {
                                new_row.push('L');
                                changes = true;
                            } else {
                                new_row.push('#');
                            }
                        },
                        '.' => { new_row.push('.'); },
                        _ => {println!("Found other");},
                    }
                    y += 1;
                }
                println!("{}", row);
                new_data.push(new_row);   
                x += 1;
                y = 0;
            }
            println!("Tries: {} Made changes: {}", tries, changes);
            println!("------------------------------------");
            data = new_data.clone();
            new_data.clear();
            x = 0;
            y = 0;
           
            tries += 1;
        }

        let final_seats = count_final(data);
        println!("Counted {} seats.", final_seats);
    }

    fn count_full_prox(x: i32, y: i32, data:&Vec<String>) -> i32 {
        let mut full_seats = 0;
        let mut i = x - 1;
        let mut j = y - 1;

        while j <= (y + 1) {
            if j >= 0 && j < data.len() as i32 { 
                let row = &data[j as usize];
                while i <= (x + 1) {

                    if i >= 0 && i < row.len() as i32 { 
                        let row_chars: Vec<char> = row.chars().collect();
                        let seat = row_chars[i as usize];
println!("Seat {} for i {} j {}", seat, i, j);
                        if seat == '#' {
                            full_seats += 1;
                        }
                    }
                    i += 1;
                }
            }
            j += 1;
            i = x - 1;
        }
println!("Count {} for x {} y {}", full_seats, x, y);
if full_seats > 9 { println!("WTF found too many"); }
        full_seats
    }

    fn count_final(data: Vec<String>) -> u32 {
        let mut final_count = 0;
        for row in data {
            for seat in row.chars().collect::<Vec<char>>() {
                if seat == '#' {
                    final_count += 1;
                }
            }
        }
        final_count
    }
}
