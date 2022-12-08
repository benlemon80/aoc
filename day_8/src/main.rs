fn main() {
    let grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let (left, right) = (&row[0..j], &row[j + 1..grid[0].len()]);
            let top: Vec<u32> = grid[0..i].iter().map(|r| r[j]).collect();
            let bottom: Vec<u32> = grid[i + 1..grid.len()].iter().map(|r| r[j]).collect();

            if is_tall(item, left.to_vec())
                || is_tall(item, right.to_vec())
                || is_tall(item, top)
                || is_tall(item, bottom)
            {
                visible += 1;
            }
        }
    }

    println!("{visible}")
}

fn is_tall(val: &u32, vec: Vec<u32>) -> bool {
    vec.iter().all(|x| x < val)
}
