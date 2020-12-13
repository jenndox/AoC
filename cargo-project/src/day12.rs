pub mod ferry {
    pub use crate::helper::helper_fns;

    pub fn steer(filename: &String) {
        let data: Vec<String> = helper_fns::file_into_vec(filename);
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut wp_x: i32 = 10;
        let mut wp_y: i32 = 1;

        for row in &data {
            println!("{}", row);
            let mut instr_chars: Vec<char> = row.chars().collect();
            let instr: char = instr_chars.remove(0);
            let dist: i32 = instr_chars.iter().collect::<String>().to_string().parse().unwrap();

            match instr {
                'F' => {
                    x += wp_x * dist;                    
                    y += wp_y * dist;                    
                },
                'N' => {
                    wp_y += dist;
                },
                'S' => {
                    wp_y -= dist;
                },
                'E' => {
                    wp_x += dist;
                },
                'W' => {
                    wp_x -= dist;
                },
                'L' => {
                    let mut temp_dist = dist;
                    while temp_dist > 0 {
                        let old_wp_x = wp_x;
                        let old_wp_y = wp_y;
                        if wp_x > 0 && wp_y >= 0 {
                            wp_x = -1 * old_wp_y;
                            wp_y = 1 * old_wp_x;
                        } else if wp_x > 0 && wp_y < 0 {
                            wp_x = -1 * old_wp_y;
                            wp_y = 1 * old_wp_x;
                        } else if wp_x <= 0 && wp_y >= 0 {
                            wp_x = -1 * old_wp_y;
                            wp_y = 1 * old_wp_x;
                        } else if wp_x <= 0 && wp_y < 0 {
                            wp_x = -1 * old_wp_y;
                            wp_y = 1 * old_wp_x;
                        }
                        temp_dist -= 90;
                    }
                }
                'R' => {
                    let mut temp_dist = dist;
                    while temp_dist > 0 {
                        let old_wp_x = wp_x;
                        let old_wp_y = wp_y;
                        if wp_x > 0 && wp_y >= 0 {
                            wp_x = 1 * old_wp_y;
                            wp_y = -1 * old_wp_x;
                        } else if wp_x > 0 && wp_y < 0 {
                            wp_x = 1 * old_wp_y;
                            wp_y = -1 * old_wp_x;
                        } else if wp_x <= 0 && wp_y >= 0 {
                            wp_x = 1 * old_wp_y;
                            wp_y = -1 * old_wp_x;
                        } else if wp_x <= 0 && wp_y < 0 {
                            wp_x = 1 * old_wp_y;
                            wp_y = -1 * old_wp_x;
                        }
                        temp_dist -= 90;
                    }
                },
                _ => {println!("Found other");},
            }
            println!("x value: {}, y value: {}, wp_x: {}, wp_y: {}", x, y, wp_x, wp_y);
        }

        println!("x value: {}, y value: {}, abs values added: {}", x, y, x.abs() + y.abs());
    }
}
