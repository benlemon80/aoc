fn main() {
    let priorities = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let sum: i32 = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (comp_1, comp_2) = l.split_at(l.len() / 2);

            let mut shared: Vec<char> = comp_1
                .chars()
                .filter(|c1| comp_2.chars().any(|c2| &c2 == c1))
                .collect();

            shared.sort();
            shared.dedup();

            shared
                .iter()
                .map(|c| priorities.iter().position(|d| d == c).unwrap() as i32 + 1)
                .sum::<i32>()
        })
        .sum();

    println!("{sum}");
}
