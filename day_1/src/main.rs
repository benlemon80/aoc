fn main() {
    let mut calories: Vec<i64> = include_str!("input.txt")
        .split("\r\n\r\n")
        .map(|s| s.split("\r\n").map(|s| s.parse::<i64>().unwrap()).sum())
        .collect();

    calories.sort();

    let top = calories.iter().max().unwrap();
    let total: i64 = calories.iter().rev().take(3).sum();

    println!("{}, {}", top, total)
}
