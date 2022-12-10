use std::collections::HashSet;

fn main() {
    const SIZE: usize = 10;

    let mut trail: HashSet<Point> = HashSet::new();
    let mut rope = vec![Point { x: 0, y: 0 }; SIZE];

    for line in include_str!("input.txt").lines() {
        let args: Vec<&str> = line.split_whitespace().collect();

        for _ in 0..args[1].parse().unwrap() {
            let (x, y) = match args[0] {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                _ => (0, -1),
            };

            rope[0] = rope[0].add(x, y);

            for i in 0..SIZE - 1 {
                let mut head = rope[i];
                let mut tail = rope[i + 1];

                tail.move_to(&mut head);
                rope[i + 1] = tail;
            }

            trail.insert(rope[SIZE - 1]);
        }
    }

    println!("{}", trail.len());
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, x: i32, y: i32) -> Point {
        self.x += x;
        self.y += y;
        self.clone()
    }

    fn move_to(&mut self, p: &mut Point) {
        let (dx, dy) = (p.x - self.x, p.y - self.y);

        if dx.abs() > 1 || dy.abs() > 1 {
            if dx == 0 {
                self.add(0, dy / 2);
            } else if dy == 0 {
                self.add(dx / 2, 0);
            } else {
                let x = if dx > 0 { 1 } else { -1 };
                self.add(x, 0);

                let y = if dy > 0 { 1 } else { -1 };
                self.add(0, y);
            }
        }
    }
}
