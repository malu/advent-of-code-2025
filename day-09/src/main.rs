fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let positions = input.lines().map(Pos::parse).collect::<Vec<_>>();
    let width = positions.iter().map(|p| p.x).max().unwrap() as usize + 1;
    let height = positions.iter().map(|p| p.y).max().unwrap() as usize + 1;

    let segments = positions
        .iter()
        .zip(positions.iter().cycle().skip(1))
        .collect::<Vec<_>>();

    let mut winding = vec![vec![0i8; width]; height];
    for &(&a, &b) in &segments {
        for y in b.y..=a.y {
            for x in a.x + 1..width as i64 {
                winding[y as usize][x as usize] += 1;
            }
        }
        for y in a.y..=b.y {
            for x in a.x + 1..width as i64 {
                winding[y as usize][x as usize] -= 1;
            }
        }
    }
    for &(&a, &b) in &segments {
        if a.x == b.x {
            for y in std::cmp::min(a.y, b.y)..=std::cmp::max(a.y, b.y) {
                winding[y as usize][a.x as usize] = 1;
            }
        }
        if a.y == b.y {
            for x in std::cmp::min(a.x, b.x)..=std::cmp::max(a.x, b.x) {
                winding[a.y as usize][x as usize] = 1;
            }
        }
    }

    let mut accumulated_outside_cells = vec![vec![0u16; width]; height];
    for (y, row) in winding.into_iter().enumerate() {
        let mut zeros = 0;
        for (x, v) in row.into_iter().enumerate() {
            accumulated_outside_cells[y][x] = zeros;
            if v == 0 {
                zeros += 1;
            }
        }
    }

    let mut areas = vec![];
    for (i, a) in positions.iter().enumerate() {
        for b in positions.iter().skip(i + 1) {
            areas.push((a, b, area(a, b)));
        }
    }
    areas.sort_by_key(|(_, _, area)| *area);
    areas.reverse();

    println!("Part1: {}", areas[0].2);
    'areas: for (a, b, area) in areas.iter() {
        for y in std::cmp::min(a.y, b.y)..=std::cmp::max(a.y, b.y) {
            if accumulated_outside_cells[y as usize][std::cmp::min(a.x, b.x) as usize]
                != accumulated_outside_cells[y as usize][std::cmp::max(a.x, b.x) as usize]
            {
                continue 'areas;
            }
        }

        println!("Part2: {area}");
        break;
    }
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
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
        Self { x, y }
    }
}

fn area(a: &Pos, b: &Pos) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}
