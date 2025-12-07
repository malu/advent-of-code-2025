use std::collections::HashSet;

fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut part1 = 0;

    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let beam_head = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, cell)| **cell == b'S')
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let mut active = HashSet::new();
    active.insert(beam_head);

    while !active.is_empty() {
        let mut next = HashSet::new();
        for (y, x) in &active {
            match map.get(*y).and_then(|row| row.get(*x)) {
                None => {}
                Some(b'^') => {
                    next.insert((y + 1, x - 1));
                    next.insert((y + 1, x + 1));
                    part1 += 1;
                }
                _ => {
                    next.insert((y + 1, *x));
                }
            }
        }

        active = next;
    }

    let mut timelines = vec![vec![1u64; map[0].len()]; map.len()];
    for y in (0..timelines.len() - 1).rev() {
        for x in 0..timelines[0].len() {
            if map[y][x] == b'^' {
                timelines[y][x] = timelines[y + 1][x - 1] + timelines[y + 1][x + 1];
            } else {
                timelines[y][x] = timelines[y + 1][x];
            }
        }
    }

    let part2 = timelines[beam_head.0][beam_head.1];

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}
