fn main() {
    let grid: Vec<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = 0;
    let mut scenic_score = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let left: Vec<u32> = row[0..j].iter().rev().copied().collect();
            let right: Vec<u32> = row[j + 1..grid[0].len()].to_vec();

            let top: Vec<u32> = grid[0..i].iter().rev().map(|r| r[j]).collect();
            let down: Vec<u32> = grid[i + 1..grid.len()].iter().map(|r| r[j]).collect();

            scenic_score = std::cmp::max(
                score(item, &top) * score(item, &left) * score(item, &down) * score(item, &right),
                scenic_score,
            );

            if is_tall(item, top)
                || is_tall(item, left)
                || is_tall(item, down)
                || is_tall(item, right)
            {
                visible += 1;
            }
        }
    }

    println!("{visible}, {scenic_score}");
}

fn is_tall(tree: &u32, trees: Vec<u32>) -> bool {
    trees.iter().all(|t| t < tree)
}

fn score(tree: &u32, trees: &Vec<u32>) -> u32 {
    let mut score = 0;
    for t in trees {
        score += 1;
        if t >= tree {
            break;
        }
    }
    score
}
