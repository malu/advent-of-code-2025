use std::collections::HashMap;

const N: usize = 569;

fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let nodes = input.lines().map(Node::parse).collect::<Vec<_>>();

    let mut names = HashMap::new();

    for n in nodes
        .iter()
        .flat_map(|n| n.conns.iter().copied().chain(std::iter::once(n.name)))
    {
        if names.contains_key(n) {
            continue;
        }
        names.insert(n, names.len());
    }
    let mut adjacency = vec![[0u64; N]; N];
    for n in &nodes {
        for c in &n.conns {
            adjacency[names[n.name]][names[c]] = 1;
        }
    }

    let mut paths = adjacency.clone();
    let mut last = adjacency.clone();
    for _ in 1..N {
        last = matmul(&last, &adjacency);
        for j in 0..N {
            for i in 0..N {
                paths[j][i] += last[j][i];
            }
        }
    }

    println!("Part1: {}", paths[names["you"]][names["out"]]);
    let svrfft = paths[names["svr"]][names["fft"]];
    let fftdac = paths[names["fft"]][names["dac"]];
    let dacout = paths[names["dac"]][names["out"]];
    let svrdac = paths[names["svr"]][names["dac"]];
    let dacfft = paths[names["dac"]][names["fft"]];
    let fftout = paths[names["fft"]][names["out"]];
    println!(
        "Part2: {}",
        svrdac * dacfft * fftout + svrfft * fftdac * dacout
    );
}

fn matmul(a: &[[u64; N]], b: &[[u64; N]]) -> Vec<[u64; N]> {
    let mut result = vec![[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

struct Node<'a> {
    name: &'a str,
    conns: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn parse(line: &'a str) -> Self {
        let (name, conns) = line.split_once(": ").unwrap();
        let conns = conns.split(" ").collect();
        Self { name, conns }
    }
}
