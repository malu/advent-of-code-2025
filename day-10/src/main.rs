fn main() {
    let input =
        std::fs::read_to_string(std::env::args().nth(1).expect("missing filename argument"))
            .expect("failed to open file");

    let lines = input.lines().map(Line::parse).collect::<Vec<_>>();

    let part1: u64 = lines.iter().map(Line::part1).sum();
    println!("Part1: {part1}");
    let part2: u64 = lines
        .iter()
        .enumerate()
        .inspect(|(i, _)| println!("====== {i}/{}", lines.len()))
        .map(|(_, line)| line.part2())
        .sum();
    println!("Part2: {part2}");
}

#[derive(Debug)]
struct Line {
    target: u64,
    buttons: Vec<u64>,
    joltages: Vec<u16>,
}

impl Line {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let target = parts
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .rev()
            .fold(0u64, |acc, b| match *b {
                b'.' => acc << 1,
                b'#' => acc << 1 | 1,
                _ => acc,
            });
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        for x in parts {
            let is_button = x.as_bytes()[0] == b'(';
            let x = &x[1..x.len() - 1];
            if is_button {
                let mut acc = 0;
                for b in x.split(',').map(|x| x.parse::<u32>().unwrap()) {
                    acc |= 1 << b;
                }
                buttons.push(acc);
            } else {
                joltages = x.split(',').map(|x| x.parse::<u16>().unwrap()).collect();
            }
        }

        Self {
            target,
            buttons,
            joltages,
        }
    }

    fn part1(&self) -> u64 {
        let mut best = u64::MAX;
        for i in 0u64..(1 << self.buttons.len()) {
            let mut outcome = 0;
            let buttons = i.count_ones() as u64;
            if buttons >= best {
                continue;
            }
            for (b, button) in self.buttons.iter().enumerate() {
                if (i & 1 << b) > 0 {
                    outcome ^= button;
                }
            }

            if outcome == self.target {
                best = buttons;
            }
        }

        best
    }

    fn part2(&self) -> u64 {
        let buttons = self
            .buttons
            .iter()
            .map(|button| {
                let mut button = *button;
                let mut out = [0u16; 12];
                let mut i = 0;
                while button > 0 {
                    if button & 1 > 0 {
                        out[i] += 1;
                    }
                    i += 1;
                    button /= 2;
                }
                out
            })
            .collect::<Vec<_>>();

        let mut matrix = vec![vec![0i32; self.buttons.len() + 1]; self.joltages.len()];
        for (i, joltage) in self.joltages.iter().enumerate() {
            matrix[i][self.buttons.len()] = *joltage as i32;
        }
        for (i, button) in buttons.iter().enumerate() {
            for j in 0..self.joltages.len() {
                matrix[j][i] = button[j] as i32;
            }
        }

        let print_matrix = |matrix: &Vec<Vec<_>>| {
            for row in matrix {
                for cell in row {
                    print!("{cell: >3}  ");
                }
                println!();
            }
        };

        let swap_rows = |matrix: &mut Vec<Vec<i32>>, i: usize, j: usize| {
            if i == j {
                return;
            }
            let [a, b] = matrix.get_disjoint_mut([i, j]).unwrap();
            std::mem::swap(a, b);
        };

        let sub_rows = |matrix: &mut Vec<Vec<i32>>, i: usize, j: usize| {
            let [a, b] = matrix.get_disjoint_mut([i, j]).unwrap();
            a.iter_mut().zip(b).for_each(|(v, w)| *v -= *w);
        };

        let scale_row = |matrix: &mut Vec<Vec<i32>>, i: usize, f: i32| {
            if f == 1 {
                return;
            }
            matrix[i].iter_mut().for_each(|v| *v *= f);
        };

        print_matrix(&matrix);
        println!();

        let mut skip_rows = 0;
        for column in 0..self.buttons.len() {
            let mut with_leading_nonzero = matrix
                .iter()
                .map(|row| row[column])
                .filter(|v| *v != 0)
                .peekable();
            if with_leading_nonzero.peek().is_none() {
                continue;
            }
            let mut lcm_ = 1;
            for v in with_leading_nonzero {
                lcm_ = lcm(lcm_, v);
            }

            // find all below row with a nonzero coefficient in this column
            let with_leading_one = matrix
                .iter()
                .enumerate()
                .skip(skip_rows)
                .filter(|(_, row)| row[column] != 0)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if !with_leading_one.is_empty() {
                let f = lcm_ / matrix[with_leading_one[0]][column];
                scale_row(&mut matrix, with_leading_one[0], f);

                for row in with_leading_one.iter().skip(1) {
                    let f = lcm_ / matrix[*row][column];
                    scale_row(&mut matrix, *row, f);
                    sub_rows(&mut matrix, *row, with_leading_one[0]);
                }

                // find all above row with a nonzero coefficient in this column
                for row in 0..skip_rows {
                    if matrix[row][column] == 0 {
                        continue;
                    }

                    let f = lcm_ / matrix[row][column];
                    scale_row(&mut matrix, row, f);
                    sub_rows(&mut matrix, row, with_leading_one[0]);
                }

                swap_rows(&mut matrix, skip_rows, with_leading_one[0]);
                skip_rows += 1;
            }
        }

        println!("Simplified matrix:");
        print_matrix(&matrix);
        println!();

        let mut ranges = vec![(0, 300); self.buttons.len()];

        // Find an upper bound on all ranges
        'ranges: for (b, range) in ranges.iter_mut().enumerate() {
            for j in range.0..=range.1 {
                let mut outcome = vec![0; self.joltages.len()];
                for i in 0..self.joltages.len() {
                    outcome[i] = j as u16 * buttons[b][i];
                }

                if outcome.iter().zip(&self.joltages).any(|(o, j)| *o > *j) {
                    range.1 = j - 1;
                    continue 'ranges;
                }
            }
        }

        println!("Solving");

        let free_columns = (0..ranges.len())
            .filter(|i| {
                for row in &matrix {
                    if row
                        .iter()
                        .enumerate()
                        .find(|(_, v)| **v != 0)
                        .map(|(c, _)| c)
                        .unwrap_or(usize::MAX)
                        == *i
                    {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        println!("free: {free_columns:?}");
        println!("{ranges:?}");

        if free_columns.is_empty() {
            if !shrink_ranges(&matrix, &mut ranges) {
                println!("no solution");
                panic!();
            }

            let count = ranges.iter().map(|(lo, _)| *lo).sum::<i32>() as u64;
            println!("fully determined; solution {count}");
            return count;
        }

        let default_ranges = ranges.clone();

        let mut best = u64::MAX;

        let value_space = ranges
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| free_columns.contains(i))
            .collect::<Vec<_>>();

        let mut assignments = value_space
            .iter()
            .map(|(i, (lo, _))| (*i, *lo))
            .collect::<Vec<_>>();

        let next_assignment = |assignments: &mut [(usize, i32)],
                               value_space: &[(usize, (i32, i32))]| {
            for (a, vs) in assignments.iter_mut().zip(value_space) {
                if a.1 == vs.1.1 {
                    a.1 = vs.1.0;
                    continue;
                } else {
                    a.1 += 1;
                    return true;
                }
            }

            false
        };

        loop {
            default_ranges.clone_into(&mut ranges);
            for (var, assignment) in &assignments {
                ranges[*var] = (*assignment, *assignment);
            }

            if !shrink_ranges(&matrix, &mut ranges) {
                if !next_assignment(&mut assignments, &value_space) {
                    break;
                }
                continue;
            }

            let cost = ranges.iter().map(|(lo, _)| lo).sum::<i32>() as u64;
            if cost >= best {
                if !next_assignment(&mut assignments, &value_space) {
                    break;
                }
                continue;
            }

            best = best.min(cost);

            if !next_assignment(&mut assignments, &value_space) {
                break;
            }
        }

        if best == u64::MAX {
            println!("no solution");
            panic!();
        }

        println!("solution: {best}");
        best
    }
}

fn shrink_ranges(matrix: &[Vec<i32>], ranges: &mut [(i32, i32)]) -> bool {
    let last = ranges.len();
    for (r, row) in matrix.iter().enumerate() {
        let mut other = 0;
        let mut undetermined = None;
        for (col, v) in row.iter().take(ranges.len()).enumerate().skip(r) {
            if *v == 0 {
                continue;
            }
            if ranges[col].0 == ranges[col].1 {
                other += *v * ranges[col].0;
                continue;
            }

            // there can be only one undetermined column per row because we fully simplified the
            // matrix
            undetermined = Some((col, *v));
        }

        if let Some((col, v)) = undetermined {
            let target = row[last] - other;
            let determined = target / v;
            if determined < ranges[col].0 || ranges[col].1 < determined || determined * v != target
            {
                return false;
            }
            ranges[col] = (target / v, target / v);
        }
    }

    true
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    while x != y {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    x
}

fn lcm(x: i32, y: i32) -> i32 {
    let x = x.abs();
    let y = y.abs();
    (x * y) / gcd(x, y)
}
