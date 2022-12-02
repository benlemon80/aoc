fn main() {
    let input = include_str!("input.txt");
    let options = ["AX", "BY", "CZ"];

    let games = input.lines().map(|game| {
        let rps: Vec<i32> = game
            .split_whitespace()
            .map(|h| options.iter().position(|&d| d.contains(h)).unwrap() as i32)
            .collect();
        (rps[0], rps[1])
    });

    let (mut p1, mut p2) = (0, 0);

    for (elf, you) in games {
        if (elf + 1) % 3 == you {
            p1 += 6;
        } else if elf == you {
            p2 += 3;
        }

        let mut res = elf + 3;

        if you == 0 {
            res = (elf + 2) % 3;
        } else if you == 2 {
            res = (elf + 1) % 3 + 6;
        }

        p1 += you + 1;
        p2 += res + 1;
    }

    println!("{}, {}", p1, p2)
}
