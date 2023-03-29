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
        let crosses: Vec<_> = row.iter().filter(|a| a.0 == '+').collect();
        println!("the line {:?} has {:?} crosses", row, crosses.len());
        if crosses.len() == 0 {
            continue;
        }

        'inner: for (y, c) in crosses.windows(crosses.len()).enumerate() {
            let left = c[0];
            let right = c[1];
            println!("my new left {:?}", left);
            println!("my new right {:?}", right);

            let is_first_row_valid = row
                .clone()
                .drain(left.2..c.last().unwrap().2)
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
                        continue 'inner;
                    }
                }
            }

            println!("\tsides are valid");

            n += 1;
        }
    }

    println!("\tsquares: {n}");

    println!("\n");
    0 as u32
}
