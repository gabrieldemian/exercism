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

    // validate rectangles recursively given a Vec
    // of Items and the number of edges.
    fn validate_rectangles(matrix: &Vec<Vec<Item>>, row_i: usize, valid_rectangles: u32) -> u32 {
        let row = matrix.get(row_i);
        let next_row = matrix.get(row_i + 1);

        if row.is_none() || next_row.is_none() {
            return valid_rectangles;
        }

        let row_has_cross = matrix[row_i].iter().any(|a| a.0 == '+');

        if !row_has_cross {
            return validate_rectangles(matrix, row_i + 1, valid_rectangles);
        }

        // this row already contains a cross
        let row = matrix[row_i].clone();
        let mut valid_rectangles = valid_rectangles;

        println!("row: {:?}\n", row);
        println!("row_i: {row_i}\n");

        let crosses: Vec<_> = row.iter().filter(|c| c.0 == '+').collect();
        let edges = crosses.len();
        println!("\nedges: {edges}");

        for (i, c) in crosses.windows(edges).enumerate() {
            let left = c.first().unwrap();
            let right = c.last().unwrap();

            println!("left {:?}", left);
            println!("right {:?}", right);

            let is_first_row_valid = row
                .clone()
                .drain(left.2..right.2)
                .all(|p| p.0 == '+' || p.0 == '-');

            println!("row {:?}", row);
            println!("\tis_valid? {:?}", is_first_row_valid);

            if !is_first_row_valid {
                continue;
            }
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
                        continue;
                    }
                }
            }

            println!("\tsides are valid");
            valid_rectangles += 1;
            println!("valid_rectangles {valid_rectangles}");
        }

        validate_rectangles(matrix, row_i + 1, valid_rectangles)
    }

    let n = validate_rectangles(&matrix, 0, 0);

    println!("\tsquares: {n}");

    println!("\n");
    0 as u32
}
