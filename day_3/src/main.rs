fn main() {
    let rucksacks: Vec<&str> = include_str!("input.txt").lines().collect();

    let priority = |c: char| -> i32 {
        let diff = if c.is_uppercase() { 38 } else { 96 };
        c as i32 - diff
    };

    let part_1 = rucksacks.iter().fold(0, |acc, l| {
        let (left, right) = l.split_at(l.len() / 2);

        let shared: Vec<char> = left
            .chars()
            .filter(|c| right.chars().any(|x| &x == c))
            .collect();

        acc + priority(shared[0])
    });

    let part_2 = rucksacks.chunks(3).fold(0, |acc, g| {
        let badge: Vec<char> = g[0]
            .chars()
            .filter(|c| g[1].chars().any(|x| &x == c) && g[2].chars().any(|x| &x == c))
            .collect();

        acc + priority(badge[0])
    });

    println!("{part_1}, {part_2}");
}
