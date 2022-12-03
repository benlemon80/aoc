fn main() {
    let priorities = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let rucksacks: Vec<&str> = include_str!("input.txt").lines().collect();

    let part_1: i32 = rucksacks.iter().fold(0, |acc, l| {
        let (one, two) = l.split_at(l.len() / 2);

        let shared: Vec<char> = one
            .chars()
            .filter(|c1| two.chars().any(|c2| &c2 == c1))
            .collect();

        acc + priorities.iter().position(|&d| d == shared[0]).unwrap() as i32 + 1
    });

    let part_2: i32 = rucksacks.chunks(3).fold(0, |acc, g| {
        let badge: Vec<char> = g[0]
            .chars()
            .filter(|c1| g[1].chars().any(|c2| &c2 == c1) && g[2].chars().any(|c3| &c3 == c1))
            .collect();

        acc + priorities.iter().position(|&d| d == badge[0]).unwrap() as i32 + 1
    });

    println!("{}, {}", part_1, part_2);
}
