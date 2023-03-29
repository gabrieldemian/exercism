type Item = (char, usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0 as u32;
    };

    //    matrix[x][y] or matrix[row][col]
    let mut matrix: Vec<Vec<Item>> = vec![];
    let mut n: u32 = 0;

    lines.into_iter().enumerate().for_each(|(row, v)| {
        matrix.push(vec![]);
        v.chars().enumerate().for_each(|(col, c)| {
            matrix[row].push((c, row, col));
        });
    });

    println!("matrix {:?}", matrix);

    'outer: for (i, row) in matrix.iter().enumerate() {
        let left = row.iter().find(|a| a.0 == '+');
        if left.is_none() {
            continue;
        };

        // find pair of + on the curr row
        let left = left.unwrap();
        let right = row.iter().skip(left.2 + 1).find(|a| a.0 == '+');

        if right.is_none() {
            continue;
        };

        let right = right.unwrap();

        let is_first_row_valid = row
            .clone()
            .drain(left.2..right.2)
            .all(|p| p.0 == '+' || p.0 == '-');

        println!("first row is {:?}", row);

        if !is_first_row_valid {
            continue;
        }

        println!("\tis_valid? {:?}", is_first_row_valid);

        let last_row = matrix
            .iter()
            .skip(i + 1)
            .find(|r| r[left.2].0 == '+' || r[right.2].0 == '+');

        println!("last row is {:?}", last_row);

        if last_row.is_none() {
            continue;
        }

        if let Some(last_row) = last_row {
            let is_valid = last_row
                .clone()
                .drain(left.2..right.2)
                .all(|c| c.0 == '+' || c.0 == '-');
            println!("\tis valid? {:?}", is_valid);
            if !is_valid {
                continue;
            }
        }

        println!("about to validate left {:?}", left);
        println!("about to validate right {:?}", right);

        // validate sides of the rectangle,
        // return the last row
        for row in matrix.iter().skip(i + 1) {
            println!("\tside left {:?}", row[left.2]);
            println!("\tside right {:?}", row[right.2]);
            match (row[left.2].0, row[right.2].0) {
                ('|', '|') => {}
                ('+', '+') => {
                    break;
                }
                _ => {
                    println!("\tsides are not valid");
                    continue 'outer;
                }
            }
        }

        println!("\tsides are valid");

        n += 1;
    }

    println!("squares: {n}");

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

    println!("\n");
    0 as u32
}
