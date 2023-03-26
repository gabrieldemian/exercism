pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0 as u32;
    };

    let mut matrix: Vec<(char, usize, usize)> = vec![];

    lines.into_iter().enumerate().for_each(|(i, v)| {
        v.chars().enumerate().for_each(|(p, c)| {
            if c == '+' || c == '-' || c == '|' {
                matrix.push((c, i, p));
            };
        });
    });

    // 0 - parse chars into the matrix
    //
    // testing the edges
    // 1 - the largest R and C of the three first +tuples equals to the last?
    // if yes, put those 4 +tuples into a subarray. The edges are valid.
    //
    // testing the borders
    // 2 - on each R, each char must be a - (except for the +)
    // 3 - on each C, each char must be a | (except for the +)

    // x(0,0) -(0,1) -(0,2) +(0,3)
    // |(1,0)               |(1,3)
    // x(2,0) -(2,1) -(2,2) +(2,3)
    // 1 - Yes

    // (1,0) (1,1) x(1,2) x(1,3)
    // (2,0) (2,1) x(2,2) x(2,3)
    // 1 - Yes
    //
    //       (0,2) (0,4)
    // (2,0) (2,2) (2,4)
    // (4,0) (4,2) (4,4)

    0 as u32
}
