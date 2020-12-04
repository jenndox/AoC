pub mod sled {
    pub use crate::helper::helper_fns;

    pub fn toboggan(filename: &String) {
        let data: [[char; 31]; 323] = helper_fns::file_into_array(filename);
        println!("Matrix is M: {} by N:{}", 31, 323);
        let mut treecut = 0;
        let mut x = 0;
        let mut y = 0;
        while y < 323 {
            println!("{} th step: {}", y, data[y][x]);
            let mut tree = data[y][x];
            if tree == '#'
            {
                treecut += 1;
            }

            y += 2;
            x += 1;
            if (x >= 31) {
                x = x % 31;
            }
        }
        println!("Counted {} trees.", treecut);
    }
}