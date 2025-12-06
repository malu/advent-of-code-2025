fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut part1 = 0;
    let mut part2 = 0;

    let lines = input.lines().collect::<Vec<_>>();
    let problems = problems(lines);

    for problem in problems {
        let (op, problem) = problem.split_last().expect("malformed input");
        let op = op[0];
        let numbers = parse_problem(problem);
        part1 += apply(op, &numbers);
        let numbers = parse_problem(&transpose(problem));
        part2 += apply(op, &numbers);
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn problems(input: Vec<&str>) -> Vec<Vec<Vec<u8>>> {
    let mut result = Vec::new();

    let mut last_cut = 0;
    for i in 0..input[0].len() {
        if i <= last_cut {
            continue;
        }

        if last_cut >= input[0].len() {
            break;
        }

        if input.iter().all(|line| line.as_bytes()[i] == b' ') {
            let mut problem = Vec::new();
            for line in input.iter() {
                problem.push(line.as_bytes()[last_cut..i].to_vec());
            }
            last_cut = i + 1;
            result.push(problem);
        }
    }

    let mut problem = Vec::new();
    for line in input.iter() {
        problem.push(line.as_bytes()[last_cut..].to_vec());
    }
    result.push(problem);

    result
}

fn apply(op: u8, values: &[u64]) -> u64 {
    match op {
        b'+' => values.iter().sum(),
        b'*' => values.iter().product(),
        b => panic!("malformed input: {b}"),
    }
}

fn parse_problem(input: &[Vec<u8>]) -> Vec<u64> {
    input.iter().map(|line| parse_number(line)).collect()
}

fn parse_number(input: &[u8]) -> u64 {
    let mut r = 0;
    for b in input {
        if !b.is_ascii_digit() {
            continue;
        }
        r = 10 * r + u64::from(b - b'0');
    }
    r
}

fn transpose(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    let height = input.len();
    let width = input[0].len();

    for x in (0..width).rev() {
        let mut row = Vec::new();
        for y in (0..height).rev() {
            row.push(input[height - y - 1][x]);
        }

        result.push(row);
    }

    result
}
