type State = i16;
const INITIAL: State = 50;

fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let mut state = INITIAL;
    let mut part1 = 0;
    let mut part2 = 0;
    for diff in input.lines().map(parse) {
        let before = state;

        state += diff;
        if state <= 0 && before > 0 {
            part2 += 1;
        }
        part2 += state.abs() / 100;
        state = state.rem_euclid(100);

        if state == 0 {
            part1 += 1;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn parse(inst: &str) -> State {
    let (direction, amount) = inst.split_at_checked(1).expect("malformed input");
    let amount: State = amount.parse().expect("malformed input");
    match direction {
        "L" => -amount,
        "R" => amount,
        _ => panic!("malformed input"),
    }
}
