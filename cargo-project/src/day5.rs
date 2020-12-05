pub mod seats {
    pub use crate::helper::helper_fns;

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Assignment {
        row: Vec<char>,
        col: Vec<char>,
        id: i64
    }

    impl Assignment {
        pub fn new(row: Vec<char>, col: Vec<char>, id: i64) -> Self {
            Assignment {
                row,
                col,
                id
            }
        }
    }

    pub fn seat_sort(filename: &String) {
        let data = helper_fns::file_into_vec(filename);
        let mut assignments: Vec<Assignment> = Vec::new();

        for x in &data {
            let pass: Vec<_> = x.chars().collect();
            let mut pass_parts = pass.into_iter().peekable();
            let row: Vec<char> = pass_parts.by_ref().take(7).collect();
            assignments.push(Assignment::new(row, pass_parts.collect(), 0));
        }

        // Sort assignments
        assignments.sort_by(|a, b| b.col.cmp(&a.col));
        assignments.sort_by(|a, b| a.row.cmp(&b.row));

        let mut candidate = 0;
        let mut suggestion = 0;
        for mut seat in assignments {
            seat.id = get_seat_id(&seat);
            if candidate - seat.id != 1 {
                suggestion = candidate - 1;
                println!("Your seat may be: {}", suggestion);
            }
            candidate = seat.id;
        }

        println!("Try seat: {}", suggestion);
    }
    pub fn seat_sort_1(filename: &String) {
        let data = helper_fns::file_into_vec(filename);
        let mut assignments: Vec<Assignment> = Vec::new();

        for x in &data {
            let pass: Vec<_> = x.chars().collect();
            let mut pass_parts = pass.into_iter().peekable();
            let row: Vec<char> = pass_parts.by_ref().take(7).collect();
            assignments.push(Assignment::new(row, pass_parts.collect(), 0));
        }

        // Sort assignments
        assignments.sort_by(|a, b| b.col.cmp(&a.col));
        assignments.sort_by(|a, b| a.row.cmp(&b.row));

        for seat in &assignments {
            println!("{:?}", seat);
        }

        let top_candidate = &assignments[0];
        println!("Top assignment: {:?}", top_candidate);

        get_seat_id(top_candidate);
    }

    fn get_seat_id(seat: &Assignment) -> i64 {
        let top_row = get_seat_row(seat);
        let top_col = get_seat_col(seat);

        let id = top_row * 8 + top_col;
        println!("answer one: {:?}", id);
        id
    }

    fn get_seat_row(seat: &Assignment) -> i64 {
        let pow = 6;
        let base: i64 = 2;
        let mut i: u32 = 0;
        let mut val: i64 = 0;
        while i <= pow {
            if seat.row[i as usize] == 'B' {
                val += base.pow(pow-i);
            }
            i += 1;
        }

        val
    }

    fn get_seat_col(seat: &Assignment) -> i64 {
        let pow = 2;
        let base: i64 = 2;
        let mut i: u32 = 0;
        let mut val: i64 = 0;
        while i <= pow {
            if seat.col[i as usize] == 'R' {
                val += base.pow(pow-i);
            }
            i += 1;
        }

        val
    }
}
