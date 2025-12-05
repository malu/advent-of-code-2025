fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut lines = input.lines();
    let mut fresh = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|range| {
            let (lo, hi) = range.split_once('-').expect("malformed input");
            let lo: u64 = lo.parse().expect("malformed input");
            let hi: u64 = hi.parse().expect("malformed input");
            lo..=hi
        })
        .collect::<Vec<_>>();

    fresh.sort_by_key(|range| *range.start());

    let mut i = 0;
    loop {
        let Some(l) = fresh.get(i) else { break };
        let Some(r) = fresh.get(i + 1) else { break };

        let overlap = l.start() <= r.end() && l.end() >= r.start()
            || r.start() <= l.end() && r.end() >= l.start();
        if overlap {
            let union = std::cmp::min(*l.start(), *r.start())..=std::cmp::max(*l.end(), *r.end());
            fresh[i] = union;
            fresh.remove(i + 1);
        } else {
            i += 1;
        }
    }

    for ingredient in lines {
        let ingredient = ingredient.parse().expect("malformed input");
        if fresh.iter().any(|range| range.contains(&ingredient)) {
            part1 += 1;
        }
    }

    for range in fresh {
        part2 += range.end() - range.start() + 1;
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}
