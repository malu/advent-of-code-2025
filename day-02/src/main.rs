fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut part1 = 0;
    let mut part2 = 0;
    let input = input.trim();
    let ranges = input.split(',');
    for range in ranges {
        let (lo, hi) = range.split_once('-').expect("malformed input");
        let lo: u64 = lo.parse().expect("malformed input");
        let hi: u64 = hi.parse().expect("malformed input");

        let (mut cur, mut cur_segments) = next2(lo);
        while cur <= hi {
            if cur_segments == 2 {
                part1 += cur;
            }
            part2 += cur;

            (cur, cur_segments) = next2(cur + 1);
        }
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn next2(cur: u64) -> (u64, u32) {
    let mut min_segments = 2;
    let mut min = next(cur, 2);
    for segments in [3, 5, 7] {
        let n = next(cur, segments);
        if n < min {
            min = n;
            min_segments = segments;
        }
    }

    (min, min_segments)
}

fn next(mut n: u64, segments: u32) -> u64 {
    let digits = n.ilog10() + 1;
    if !digits.is_multiple_of(segments) {
        return next(10u64.pow(digits), segments);
    }

    let f = 10u64.pow(digits / segments);
    let mut scatter = 0;

    let mut x = n % f;
    while n > 0 {
        scatter *= f;
        scatter += 1;

        if n % f < x {
            x = (n % f) + 1;
        } else {
            x = n % f;
        }
        n /= f;
    }

    x * scatter
}

#[cfg(test)]
mod tests {
    #[test]
    fn next() {
        assert_eq!(super::next(99, 2), 99);
        assert_eq!(super::next(100, 2), 1010);
        assert_eq!(super::next(100, 3), 111);
        assert_eq!(super::next(120000, 2), 120120);
        assert_eq!(super::next(212121212118, 2), 212121212121);
        assert_eq!(super::next(212121212121, 2), 212121212121);
        assert_eq!(super::next(120000, 3), 121212);
        assert_eq!(super::next(212121212118, 6), 212121212121);
        assert_eq!(super::next(212121212121, 6), 212121212121);
    }
}
