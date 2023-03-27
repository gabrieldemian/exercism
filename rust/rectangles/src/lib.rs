type Item = (char, usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0 as u32;
    };

    //    matrix[x][y] or matrix[row][col]
    let mut matrix: Vec<Vec<Item>> = vec![];

    lines.into_iter().enumerate().for_each(|(row, v)| {
        matrix.push(vec![]);
        v.chars().enumerate().for_each(|(col, c)| {
            matrix[row].push((c, row, col));
        });
    });

    println!("matrix {:?}", matrix);

    let m_iter = matrix.into_iter().enumerate();

    let mut candidates: Vec<Item> = vec![];

    for (_r, row) in m_iter {
        let mut prev_col: Option<Item> = None;

        for col_data in row {
            let (symbol, row, col) = col_data;
            println!("curr {:?}", col_data);
            println!("prev {:?}", prev_col);

            if symbol == '+' {
                candidates.push(col_data);
            };

            if col == 0 {
                prev_col = Some(col_data);
                continue;
            };

            if let Some(prev) = prev_col {
                match prev.0 {
                    '+' if symbol != '-' && symbol != '+' => {
                        candidates.remove(row + col);
                    }
                    '-' if symbol == ' ' => {
                        candidates.remove(row + col);
                    }
                    ' ' if symbol == '-' => {
                        candidates.remove(row + col);
                    }
                    _ => {}
                };
            }
            println!("candidates {:?}", candidates);
            prev_col = Some(col_data);
        }
    }

    for all in candidates.windows(4).into_iter() {
        let (a, b, c, d) = (all[0], all[1], all[2], all[3]);
        // all cols from a to b -> are -
        // all cols from a to c \/ are |
        // all cols from c -> d are |

        for col in a.2..=b.2 {
            // matrix[a.1]
        }

        println!("all {:?}", all);
    }

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
    // alternative solution?
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

    // let n = (candidates.iter().count() / 4) as f64;
    // n.floor() as u32
    0 as u32
}
