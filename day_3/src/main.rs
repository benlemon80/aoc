fn main() {
    let rucksacks: Vec<&str> = include_str!("input.txt").lines().collect();

    let part_1 = rucksacks.iter().fold(0, |acc, l| {
        let (one, two) = l.split_at(l.len() / 2);

        let shared: Vec<char> = one
            .chars()
            .filter(|c1| two.chars().any(|c2| &c2 == c1))
            .collect();

        acc + priority(shared[0])
    });

    let part_2 = rucksacks.chunks(3).fold(0, |acc, g| {
        let badge: Vec<char> = g[0]
            .chars()
            .filter(|c1| g[1].chars().any(|c2| &c2 == c1) && g[2].chars().any(|c3| &c3 == c1))
            .collect();

        acc + priority(badge[0])
    });

    println!("{}, {}", part_1, part_2);
}

fn priority(c: char) -> i32 {
    let diff = if c.is_uppercase() { 38 } else { 96 };
    c as i32 - diff
}
