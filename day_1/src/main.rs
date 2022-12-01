use std::fs;

fn main() {
    let calories: i64 = fs::read_to_string("./input.txt")
        .expect("Could not read input!")
        .split("\r\n\r\n")
        .map(|s| s.split("\r\n").map(|s| s.parse::<i64>().unwrap()).sum())
        .max()
        .unwrap();

    println!("{}", calories);
}
