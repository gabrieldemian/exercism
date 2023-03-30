type Item = (char, usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0 as u32;
    };

    //    matrix[x][y] or matrix[row][col]
    let mut matrix: Vec<Vec<Item>> = vec![];
    let mut n = 0 as u32;

    lines.into_iter().enumerate().for_each(|(row, v)| {
        matrix.push(vec![]);
        v.chars().enumerate().for_each(|(col, c)| {
            matrix[row].push((c, row, col));
        });
    });

    println!("matrix {:?}", matrix);

    // validate rectangles recursively given a Vec
    // of Items and the number of edges.

    'main: for (y, row) in matrix.iter().enumerate() {
        if y == matrix.len() - 1 {
            break;
        }
        println!("");
        println!("-- started new row --");
        // vector of +
        let crosses: Vec<_> = row.iter().filter(|c| c.0 == '+').collect();
        // quantity of edges on curr row
        let mut edges = 2;

        println!("row {:?}", row);

        'outer: loop {
            'inner: for (i, c) in crosses.windows(edges).enumerate() {
                println!("\t-- validating cols between pairs of + --");
                println!("\tpassing on pair of i: {}", i);
                println!("\tpair is: {:?}", c);

                let left = c.first();
                if let None = left {
                    break 'outer;
                }
                let left = c.first().unwrap();
                let right = c.last().unwrap();

                let is_first_row_valid = row
                    .clone()
                    .drain(left.2..right.2)
                    .all(|p| p.0 == '+' || p.0 == '-');

                println!("\tis_valid? {:?}", is_first_row_valid);
                println!("");

                if !is_first_row_valid {
                    continue;
                }

                println!("\t-- validating cols below pairs of + --");
                println!("\tleft {:?}", left);
                println!("\tright {:?}", right);

                // validate sides of the rectangle,
                // return the last row
                println!("\t y + 1 is {}", y + 1);
                println!("\t i + 1 is {}", i + 1);

                for row in matrix.iter().skip(y + 1) {
                    println!("\t-- validating row {:?} --", row);
                    println!("\tleft {:?}", row[left.2]);
                    println!("\tright {:?}", row[right.2]);

                    match (row[left.2].0, row[right.2].0) {
                        ('|', '|') => {
                            println!("\tvalid");
                        }
                        ('+', '+') => {
                            println!("\t -- found last row, validating: {:?} --", row);
                            let is_valid = row
                                .clone()
                                .drain(left.2..right.2)
                                .all(|c| c.0 == '+' || c.0 == '-');

                            if is_valid {
                                println!("\t valid rectangle, incrementing n");
                                n += 1;
                                println!("\t n is {n}");
                                println!("");
                            } else {
                                println!("\tlast row is invalid. Invalid rectangle.")
                            }
                        }
                        _ => {
                            println!("\tsides are not valid");
                            break 'inner;
                        }
                    }
                }

                println!("\tsides are valid");
            }
            if edges + 1 <= crosses.len() {
                edges += 1;
            } else {
                println!("breaking outer...");
                break 'outer;
            }
        }
    }

    println!("");
    println!("--- final result: n is: {n}");
    println!("\n");
    0 as u32
}
