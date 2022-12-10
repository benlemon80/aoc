use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input.txt");

    let mut cpu: BTreeMap<i32, i32> = BTreeMap::new();
    let (mut count, mut x) = (0, 1);

    for line in input.lines() {
        count += 1;
        cpu.insert(count, x);

        let args: Vec<&str> = line.split_whitespace().collect();

        if args.len() == 2 {
            count += 1;
            cpu.insert(count, x);

            x += args[1].parse::<i32>().unwrap();
        }
    }

    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    let mut pixels: Vec<&str> = Vec::new();

    for (c, x) in cpu.into_iter() {
        if cycles.contains(&c) {
            sum += c * x;
        }

        let pos = (c - 1) % 40;

        if (x - 1..x + 2).contains(&pos) {
            pixels.push("[]");
        } else {
            pixels.push("..");
        }
    }

    println!("part_1: {sum}\npart_2:");
    pixels.chunks(40).for_each(|c| println!("{}", c.join("")));
}
