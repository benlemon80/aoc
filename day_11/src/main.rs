fn main() {
    let mut monkeys: Vec<Monkey> = include_str!("input.txt")
        .split("\r\n\r\n")
        .map(Monkey::parse)
        .collect();

    let magic: u64 = monkeys.iter().map(|m| m.divisor).product();
    let (divide, rounds) = (true, 20);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let changes = monkey.inspect(divide, magic);

            for (j, n) in changes {
                let target = monkeys.get_mut(j).unwrap();
                target.items.push(n);
            }
        }
    }

    let mut counts: Vec<u64> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();

    println!("{}", counts.iter().rev().take(2).product::<u64>());
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: String,
    divisor: u64,
    pass: usize,
    fail: usize,
    count: u64,
}

impl Monkey {
    fn parse(chunk: &str) -> Monkey {
        let lines: Vec<&str> = chunk.split("\r\n").map(|s| s.trim_start()).collect();

        Monkey {
            items: lines[1]
                .split_at(16)
                .1
                .split(", ")
                .map(|c| c.parse().unwrap())
                .collect(),

            op: lines[2].split_at(17).1.to_string(),
            divisor: lines[3].split_whitespace().last().unwrap().parse().unwrap(),
            pass: lines[4].split_whitespace().last().unwrap().parse().unwrap(),
            fail: lines[5].split_whitespace().last().unwrap().parse().unwrap(),
            count: 0,
        }
    }

    fn inspect(&mut self, divide: bool, mod_div: u64) -> Vec<(usize, u64)> {
        let mut changes = Vec::new();

        for i in 0..self.items.len() {
            let mut worry = self.do_operation(self.items[i], mod_div);

            let div = if divide { 3 } else { 1 };
            worry = ((worry / div) as f64).round() as u64;

            let next = if worry % self.divisor == 0 {
                self.pass
            } else {
                self.fail
            };

            changes.push((next, worry));
            self.count += 1;
        }

        self.items = Vec::new();
        changes
    }

    fn do_operation(&self, old: u64, mod_div: u64) -> u64 {
        let args: Vec<&str> = self.op.split_whitespace().collect();
        let worry: u64 = match (args[0], args[1], args[2]) {
            (_, "+", "old") => old + old,
            (_, "*", "old") => old * old,
            (_, _, x) => {
                let val: u64 = x.parse().unwrap();
                if args[1] == "+" {
                    old + val
                } else {
                    old * val
                }
            }
        };
        worry % mod_div
    }
}
