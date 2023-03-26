type Item = (char, usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0 as u32;
    };

    let mut matrix: Vec<Item> = vec![];
    let mut valid_x: Vec<Item> = vec![];

    lines.into_iter().enumerate().for_each(|(row, v)| {
        v.chars().enumerate().for_each(|(col, c)| {
            if c == '+' || c == '-' || c == '|' {
                matrix.push((c, row, col));
            };
        });
    });

    let only_x: Vec<Item> = matrix.into_iter().filter(|c| c.0 == '+').collect();

    'row: for (row, row_t) in only_x.iter().enumerate() {
        let mut valid_xs: Vec<Item> = vec![row_t.clone()];
        println!("curr row_t {:?}", row_t);

        for (y, col_t) in only_x.clone().into_iter().enumerate() {
            if y == row || valid_xs.contains(&col_t) {
                continue;
            }
            println!("curr col_t {:?}", col_t);
            match valid_xs.len() {
                1 => {
                    if col_t.1 == valid_xs[0].1 || col_t.2 == valid_xs[0].2 {
                        valid_xs.push(col_t);
                        println!("inside 1 if valid_xs {:?}", valid_xs);
                    }
                }
                2 => {
                    if col_t.2 == valid_xs[0].2 || col_t.2 == valid_xs[1].2 {
                        valid_xs.push(col_t);
                        println!("inside 2 if valid_xs {:?}", valid_xs);
                    }
                }
                3 => {
                    if col_t.1 == valid_xs[2].1 {
                        valid_xs.push(col_t);
                        println!("inside 3 if valid_xs {:?}", valid_xs);
                        valid_x.append(&mut valid_xs);
                        println!("breaking loop");
                        break 'row;
                    }
                }
                _ => {}
            };
        }
    }

    println!("valid xs: {:?}", valid_x);

    // 0 - parse chars into the matrix
    //
    // # validating the edges
    // 1. select next x
    // 2. 2nd tuple must have same C or R as 1st
    // 3. the 3rd tuple C must be one of the previous tuples C
    // 4. the 4th tuple R must be the same R as the 3rd
    // 5. X the 4th tuple C must be the same C as the 2nd
    //
    // # validating the borders
    // 2. on each R, each char must be a - (except for the +)
    // 3. on each C, each char must be a | (except for the +)
    //
    // alternative solution? O(2^n)
    // the 4th tuple RC is always the largest of the previous
    // 1. get all the valid xs edges into subarrays
    // 2. 2nd tuple must have same C or R as 1st
    // 2. the 4th RC is the largest of the previous
    //
    // alternative solution?
    // count the intersections and do something

    // x(0,0) -(0,1) -(0,2) +(0,3)
    // |(1,0)               |(1,3)
    // x(2,0) -(2,1) -(2,2) +(2,3)
    // alternative - Yes

    // (1,0) (1,1) x(1,2) x(1,3)
    // (2,0) (2,1) x(2,2) x(2,3)
    // alternative - Yes
    //
    //       (0,2) (0,4)
    // (2,0) (2,2) (2,4)
    // (4,0) (4,2) (4,4)

    (valid_x.iter().count() as i32 / 4) as u32
}
