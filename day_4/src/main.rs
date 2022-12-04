fn check_overlaps(n: &Vec<i32>) -> bool {
    (n[0] < n[2] && n[1] < n[2]) || (n[2] < n[0] && n[3] < n[0])
}

fn check_contains(n: &Vec<i32>) -> bool {
    (n[0] >= n[2] && n[1] <= n[3]) || (n[2] >= n[0] && n[3] <= n[1])
}

fn main() {
    let digits: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split(&[',', '-']).map(|s| s.parse().unwrap()).collect())
        .collect();

    let part_1 = digits
        .iter()
        .fold(0, |acc, n| acc + if check_contains(&n) { 1 } else { 0 });

    let part_2 = digits
        .iter()
        .fold(0, |acc, n| acc + if check_overlaps(&n) { 0 } else { 1 });

    println!("{part_1}, {part_2}")
}
