use std::cmp::Ordering;

fn main() {
    let grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = 0;
    let mut scenic_score = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let left: Vec<u32> = row[0..j].iter().rev().map(|x| *x).collect();

            let right: Vec<u32> = row[j + 1..grid[0].len()].iter().map(|x| *x).collect();

            let top: Vec<u32> = grid[0..i].iter().rev().map(|r| r[j]).collect();

            let bottom: Vec<u32> = grid[i + 1..grid.len()].iter().map(|r| r[j]).collect();

            scenic_score = std::cmp::max(
                score(item, &top) * score(item, &left) * score(item, &bottom) * score(item, &right),
                scenic_score,
            );

            if is_tall(item, left)
                || is_tall(item, right)
                || is_tall(item, top)
                || is_tall(item, bottom)
            {
                visible += 1;
            }
        }
    }

    println!("{visible}");
    println!("{scenic_score}");
}

fn is_tall(val: &u32, vec: Vec<u32>) -> bool {
    vec.iter().all(|x| x < val)
}

fn score(val: &u32, vec: &Vec<u32>) -> u32 {
    let mut score = 0;
    for tree in vec {
        match tree.cmp(val) {
            Ordering::Less => score += 1,
            Ordering::Greater | Ordering::Equal => {
                score += 1;
                break;
            }
        }
    }
    score
}
