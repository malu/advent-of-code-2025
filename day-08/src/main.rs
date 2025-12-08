use std::collections::HashMap;

fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let iters: usize = std::env::args()
        .nth(2)
        .expect("missing number of iterations")
        .parse()
        .expect("failed to parse iters");

    let positions = input.lines().map(Pos::parse).collect::<Vec<_>>();

    let mut circuit = (0..positions.len()).collect::<Vec<_>>();
    let mut circuits = positions.len();

    let mut dists = Vec::with_capacity(positions.len() * (positions.len() - 1) / 2);
    for (i, a) in positions.iter().enumerate() {
        for (j, b) in positions.iter().enumerate().skip(i + 1) {
            dists.push((a.dist2(b), (i, j)));
        }
    }

    dists.sort_by_key(|(d, _)| *d);

    for (k, (_, (i, j))) in dists.into_iter().enumerate() {
        let (ci, cj) = (circuit[i], circuit[j]);

        if ci != cj {
            circuits -= 1;
            circuit.iter_mut().for_each(|c| {
                if *c == cj {
                    *c = ci;
                }
            });
        }

        if k == iters - 1 {
            let mut sizes = HashMap::<usize, u64>::new();
            for c in &circuit {
                *sizes.entry(*c).or_default() += 1;
            }
            let mut sizes = sizes.values().collect::<Vec<_>>();
            sizes.sort();
            sizes.reverse();

            println!("Part1: {}", sizes.into_iter().take(3).product::<u64>());
        }

        if circuits == 1 {
            println!("Part2: {}", positions[i].x * positions[j].x);
            break;
        }
    }
}

#[derive(Debug)]
struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

impl Pos {
    fn parse(input: &str) -> Self {
        let mut split = input.split(',');
        let x = split
            .next()
            .expect("malformed input")
            .parse()
            .expect("malformed input");
        let y = split
            .next()
            .expect("malformed input")
            .parse()
            .expect("malformed input");
        let z = split
            .next()
            .expect("malformed input")
            .parse()
            .expect("malformed input");
        Self { x, y, z }
    }

    fn dist2(&self, rhs: &Self) -> u64 {
        (self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2) + (self.z - rhs.z).pow(2)
    }
}
