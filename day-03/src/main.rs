fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut part1 = 0;
    let mut part2 = 0;
    let input = input.trim();

    for bank in input.lines() {
        part1 += max_joltage::<2>(bank.as_bytes());
        part2 += max_joltage::<12>(bank.as_bytes());
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn max_joltage<const N: usize>(bank: &[u8]) -> u64 {
    let mut best = [0; N];
    for (i, item) in bank.iter().map(|b| b - b'0').enumerate() {
        let remaining = bank.len() - i;
        let offset = N.saturating_sub(remaining);
        for j in offset..N {
            if item > best[j] {
                best[j] = item;
                best[j + 1..].fill(0);
                break;
            }
        }
    }

    best.into_iter().fold(0, |t, i| 10 * t + i as u64)
}
