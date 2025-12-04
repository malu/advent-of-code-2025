fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut map = input
        .trim()
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| *b == b'@')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut first_iter = true;
    loop {
        let mut to_clear = Vec::new();
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell && neighbors(&map, x, y) < 4 {
                    to_clear.push((x, y));
                    if first_iter {
                        part1 += 1;
                    }
                    part2 += 1;
                }
            }
        }

        first_iter = false;
        if to_clear.is_empty() {
            break;
        }

        for (x, y) in to_clear {
            map[y][x] = false;
        }
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn roll_at(map: &[Vec<bool>], x: usize, y: usize) -> Option<bool> {
    map.get(y)?.get(x).copied()
}

fn neighbors(map: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut count = 0;

    let coordinates = [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x.wrapping_sub(1), y.wrapping_add(0)),
        (x.wrapping_sub(1), y.wrapping_add(1)),
        (x.wrapping_add(0), y.wrapping_sub(1)),
        (x.wrapping_add(0), y.wrapping_add(1)),
        (x.wrapping_add(1), y.wrapping_sub(1)),
        (x.wrapping_add(1), y.wrapping_add(0)),
        (x.wrapping_add(1), y.wrapping_add(1)),
    ];
    for (x, y) in coordinates {
        if roll_at(map, x, y) == Some(true) {
            count += 1;
        }
    }

    count
}
