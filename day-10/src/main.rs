#![feature(portable_simd)]

use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdInt;
use std::simd::{i32x16, u16x16};

const DEBUG: bool = false;

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
        .inspect(|(i, _)| {
            if DEBUG {
                println!("====== {i}/{}", lines.len())
            }
        })
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
                let mut out = u16x16::splat(0);
                let mut i = 0;
                while button > 0 {
                    if button & 1 > 0 {
                        out[i] = 1;
                    }
                    i += 1;
                    button /= 2;
                }
                out
            })
            .collect::<Vec<_>>();
        let joltages = u16x16::load_or_default(&self.joltages);

        let mut matrix = vec![ZERO; self.joltages.len()];
        for (i, joltage) in self.joltages.iter().enumerate() {
            matrix[i][self.buttons.len()] = *joltage as i32;
        }
        for (i, button) in buttons.iter().enumerate() {
            for j in 0..self.joltages.len() {
                matrix[j][i] = button[j] as i32;
            }
        }

        let print_matrix = |matrix: &Vec<i32x16>| {
            for row in matrix {
                for cell in &row.as_array()[..self.buttons.len() + 1] {
                    print!("{cell: >3}  ");
                }
                println!();
            }
        };

        if DEBUG {
            print_matrix(&matrix);
            println!();
        }

        let mut skip_rows = 0;
        for column in 0..self.buttons.len() {
            let pivot = matrix
                .iter()
                .enumerate()
                .skip(skip_rows)
                .filter(|(_, row)| row[column] != 0)
                .map(|(i, pivot)| (i, *pivot))
                .next();

            if let Some((pivot_index, pivot)) = pivot {
                let p = i32x16::splat(pivot[column]);

                for (i, row) in matrix.iter_mut().enumerate() {
                    if i == pivot_index || row[column] == 0 {
                        continue;
                    }

                    let f = row[column];
                    *row = *row * p - pivot * i32x16::splat(f);
                }

                matrix.swap(skip_rows, pivot_index);
                skip_rows += 1;
            }
        }

        matrix.retain(|row| *row != ZERO);

        if DEBUG {
            println!("Simplified matrix:");
            print_matrix(&matrix);
            println!();
        }

        let mut ranges = (ZERO, i32x16::splat(300));

        // Find an upper bound on all ranges
        for (b, button) in buttons.iter().enumerate() {
            for j in ranges.0[b]..=ranges.1[b] {
                let outcome = u16x16::splat(j as u16) * *button;
                if outcome.simd_gt(joltages).any() {
                    ranges.1[b] = j - 1;
                    break;
                }
            }
        }

        if DEBUG {
            println!("Solving");
        }

        let free_columns = (0..self.buttons.len())
            .filter(|i| {
                for row in &matrix {
                    if row
                        .as_array()
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
        let dependent_columns = (0..self.buttons.len())
            .filter(|i| !free_columns.contains(i))
            .collect::<Vec<_>>();
        if DEBUG {
            println!("free: {free_columns:?}");
            println!("dependent: {dependent_columns:?}");
            println!("{:?}", &ranges.1[0..self.buttons.len()]);
        }

        let default_ranges = ranges;

        let mut best = u64::MAX;

        let value_space = ranges
            .0
            .to_array()
            .iter()
            .zip(&ranges.1.to_array())
            .enumerate()
            .filter(|(i, _)| free_columns.contains(i))
            .map(|(i, (lo, hi))| (i, (*lo, *hi)))
            .collect::<Vec<_>>();

        let mut assignments = value_space
            .iter()
            .map(|(i, (lo, _))| (*i, *lo))
            .collect::<Vec<_>>();

        let next_assignment = |assignments: &mut [(usize, i32)],
                               value_space: &[(usize, (i32, i32))],
                               mut out_of_range: bool| {
            for (a, vs) in assignments
                .iter_mut()
                .zip(value_space)
                .skip_while(move |(a, vs)| out_of_range && a.1 == vs.1.0)
            {
                if a.1 == vs.1.1 || out_of_range {
                    out_of_range = false;
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
                ranges.0[*var] = *assignment;
                ranges.1[*var] = *assignment;
            }

            let cost = ranges.0.reduce_sum() as u64;
            if cost >= best {
                if !next_assignment(&mut assignments, &value_space, true) {
                    break;
                }
                continue;
            }

            let Some(cost) = solve(
                &matrix,
                &ranges,
                self.buttons.len(),
                best,
                &dependent_columns,
            ) else {
                if !next_assignment(&mut assignments, &value_space, false) {
                    break;
                }
                continue;
            };

            best = best.min(cost);

            if !next_assignment(&mut assignments, &value_space, false) {
                break;
            }
        }

        if best == u64::MAX {
            println!("no solution");
            panic!();
        }

        if DEBUG {
            println!("solution: {best}");
        }
        best
    }
}

const ZERO: i32x16 = i32x16::splat(0);

fn solve(
    matrix: &[i32x16],
    (lo, hi): &(i32x16, i32x16),
    last: usize,
    best: u64,
    dependent_indices: &[usize],
) -> Option<u64> {
    // This is effectively the set of free variables whose values we are now iterating. Each
    // dependent variable only occurs in one row of the matrix, hence we do not update this value
    // after determining a dependent variable's value.
    let determined_ranges = lo.simd_eq(*hi);
    let mut cost = lo.reduce_sum() as u64;
    for (row, &i) in matrix.iter().zip(dependent_indices) {
        let other2 = determined_ranges.select(row * *lo, ZERO).reduce_sum();
        let target = row[last] - other2;

        let v = row[i];
        if v == 1 {
            let determined = target;
            cost += determined as u64;
            if determined < lo[i] || hi[i] < determined || cost >= best {
                return None;
            }
        } else {
            let determined = target / v;
            cost += determined as u64;
            if determined < lo[i] || hi[i] < determined || determined * v != target || cost >= best
            {
                return None;
            }
        }
    }

    Some(cost)
}
