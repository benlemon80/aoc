use std::collections::{HashSet, VecDeque};

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut a_starts: Vec<(i32, i32)> = Vec::new();

    for (m, line) in include_str!("input.txt").lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (n, char) in line.chars().enumerate() {
            row.push(char);
            let pos = (m as i32, n as i32);
            match char {
                'S' => start = pos,
                'E' => end = pos,
                _ => {}
            }

            if char == 'S' || char == 'a' {
                a_starts.push(pos);
            }
        }
        grid.push(row);
    }

    println!("{}", bfs(&grid, start, end).unwrap());

    let steps = a_starts
        .iter()
        .map(|s| bfs(&grid, *s, end))
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .min()
        .unwrap();

    println!("{steps}")
}
struct N {
    pos: (i32, i32),
    dist: i32,
}

fn bfs(grid: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> Option<i32> {
    let mut queue: VecDeque<N> = VecDeque::new();
    queue.push_back(N {
        pos: start,
        dist: 0,
    });

    let mut explored: HashSet<(i32, i32)> = HashSet::new();

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if explored.contains(&node.pos) {
            continue;
        }
        explored.insert(node.pos);
        if node.pos == end {
            return Some(node.dist);
        }
        for v in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let new = (node.pos.0 + v.0, node.pos.1 + v.1);
            if is_valid_move(&grid, node.pos, new) {
                queue.push_back(N {
                    pos: new,
                    dist: node.dist + 1,
                });
            }
        }
    }
    None
}


fn is_valid_move(grid: &Vec<Vec<char>>, from: (i32, i32), to: (i32, i32)) -> bool {
    let max_rows = grid.len() as i32;
    let max_cols = grid[0].len() as i32;

    let is_on_grid = |m: i32, n: i32| -> bool { m >= 0 && n >= 0 && m < max_rows && n < max_cols };
    let is_ok_char = |cf: char, ct: char| -> bool { cf == ct || (ct as i32 - cf as i32) <= 1 };

    if is_on_grid(to.0, to.1) {
        let mut char_from = grid[from.0 as usize][from.1 as usize];
        let mut char_to = grid[to.0 as usize][to.1 as usize];

        if char_from == 'S' {
            char_from = 'a';
        }

        if char_to == 'E' {
            char_to = 'z';
        }

        return is_ok_char(char_from, char_to);
    }
    false
}
