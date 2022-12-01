fn main() {
    let mut calories: Vec<i64> = include_str!("input.txt")
        .split("\r\n\r\n")
        .map(|s| s.split("\r\n").map(|s| s.parse::<i64>().unwrap()).sum())
        .collect();

    calories.sort();

    let top_three_total: i64 = calories.iter().rev().take(3).sum();

    println!("{top_three_total}")
}
