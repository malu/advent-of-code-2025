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

fn max_joltage<const N: usize>(mut bank: &[u8]) -> u64 {
    let mut result = 0;

    for j in 0..N {
        let (i, b) = bank[..bank.len() - (N - j - 1)]
            .iter()
            .enumerate()
            // `max_by_key` returns the last maximum entry. We are seeking the first and need to
            // reverse the iteration order.
            .rev()
            .max_by_key(|(_, b)| **b)
            .unwrap();
        bank = &bank[i + 1..];
        result = 10 * result + u64::from(*b - b'0');
    }

    result
}
