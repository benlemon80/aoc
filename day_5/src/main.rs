fn main() {
    let input: Vec<&str> = include_str!("input.txt").split("\r\n\r\n").collect();

    let mut cargo: Vec<&str> = input[0].lines().collect();
    cargo.remove(cargo.len() - 1);

    let mut stacks: Vec<Vec<&str>> = (1..cargo[0].len())
        .step_by(4)
        .map(|index| {
            cargo
                .iter()
                .map(|c| &c[index..index + 1])
                .filter(|&x| x != " ")
                .rev()
                .collect()
        })
        .collect();

    input[1]
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .for_each(|step: Vec<usize>| {
            let (count, from, to) = (step[0], step[1] - 1, step[2] - 1);
            let range = stacks[from].len() - count..;
            let crates: Vec<&str> = stacks[from].drain(range).collect();
            
            // For part two, remove this .rev()
            crates.iter().rev().for_each(|c| stacks[to].push(c));
        });

    let top_crates: Vec<&str> = stacks.iter().map(|s| *s.iter().last().unwrap()).collect();
    println!("{}", top_crates.join(""))
}
