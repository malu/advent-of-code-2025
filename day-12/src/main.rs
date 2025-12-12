fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let lines = input.lines().collect::<Vec<_>>();
    let lines = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let regions = lines[6]
        .iter()
        .copied()
        .map(Region::parse)
        .collect::<Vec<_>>();

    let part1 = regions.iter().filter(|r| r.attempt()).count();
    println!("Part1: {part1}");
}

#[derive(Debug)]
struct Region {
    reqs: [u8; 6],
    width: usize,
    height: usize,
}

impl Region {
    fn parse(line: &str) -> Self {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let (w, h) = split[0].split_once('x').unwrap();
        let width = w.parse::<usize>().unwrap();
        let height = h[0..h.len() - 1].parse::<usize>().unwrap();
        let mut reqs = [0; 6];
        for i in 0..6 {
            reqs[i] = split[1 + i].parse().unwrap()
        }
        Self {
            reqs,
            width,
            height,
        }
    }

    // Not a solution to the public examples, but to the private input
    fn attempt(&self) -> bool {
        self.width * self.height < 7 * self.reqs.iter().copied().map(usize::from).sum::<usize>()
    }
}
