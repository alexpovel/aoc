use itertools::Itertools;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::VecDeque,
    ops::Range,
};

use crate::Challenge;

/// --- Part Two ---
///
/// Everyone will starve if you only plant such a small number of seeds. Re-reading the
/// almanac, it looks like the seeds: line actually describes ranges of seed numbers.
///
/// The values on the initial seeds: line come in pairs. Within each pair, the first
/// value is the start of the range and the second value is the length of the range. So,
/// in the first line of the example above:
///
/// ```text
/// seeds: 79 14 55 13
/// ```
///
/// This line describes two ranges of seed numbers to be planted in the garden. The
/// first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92.
/// The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66,
/// 67.
///
/// Now, rather than considering four seed numbers, you need to consider a total of 27
/// seed numbers.
///
/// In the above example, the lowest location number can be obtained from seed number
/// 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45,
/// humidity 46, and location 46. So, the lowest location number is 46.
///
/// Consider all of the initial seed numbers listed in the ranges on the first line of
/// the almanac. What is the lowest location number that corresponds to any of the
/// initial seed numbers?
pub struct Part {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shift {
    range: Range<usize>,
    by: isize,
}

impl Shift {
    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

type Ranges = Vec<Range<usize>>;
type Shifts = Vec<Shift>;

impl Part {}

#[inline]
fn shift_by(range: Range<usize>, by: isize) -> Range<usize> {
    range.start.checked_add_signed(by).unwrap()..range.end.checked_add_signed(by).unwrap()
}

fn apply(ranges: Ranges, shifts: Shifts) -> Ranges {
    let mut res = Vec::with_capacity(ranges.len()); // approx. size

    let mut queue = VecDeque::from(ranges);

    // While processing, ranges might be split into various pieces; these *new*, smaller
    // pieces are added back for later processing, as other shifts might apply to them.
    'q: while let Some(range) = queue.pop_front() {
        for shift in &shifts {
            match (
                shift.start().cmp(&range.start),
                shift.start().cmp(&range.end),
                shift.end().cmp(&range.start),
                shift.end().cmp(&range.end),
            ) {
                // Out of range: shift fully left of range
                (_, _, Less | Equal, _) => continue,
                // Out of range: shift fully right of range
                (_, Greater | Equal, _, _) => continue,
                // Shift fully contains range
                (Less | Equal, _, _, Greater | Equal) => {
                    res.push(shift_by(range.clone(), shift.by));
                    continue 'q;
                }
                // Partial overlap: shift left of range
                (Less | Equal, _, _, Less) => {
                    let right = shift.end()..range.end;

                    queue.push_back(right);

                    res.push(shift_by(range.start..shift.end(), shift.by));
                    continue 'q;
                }
                // Shift fully covered by range, splitting it three ways
                (Greater, _, _, Less) => {
                    let left = range.start..shift.start();
                    let right = shift.end()..range.end;

                    queue.push_back(left);
                    queue.push_back(right);

                    res.push(shift_by(shift.range.clone(), shift.by));

                    continue 'q;
                }
                // Partial overlap: shift right of range
                (Greater, _, _, Equal | Greater) => {
                    let left = range.start..shift.start();

                    queue.push_back(left);

                    res.push(shift_by(shift.start()..range.end, shift.by));

                    continue 'q;
                }
            }

            // if shift.range.start <= range.start && shift.range.end >= range.end {
            //     // Shift fully encompasses range, shift entire range
            //     res.push(shift_by(range, shift.by));

            //     continue 'outer;
            // } else if shift.range.start > range.start && shift.range.end < range.end {
            //     // Shift is fully contained in range, split
            //     let left = range.start..shift.range.start;
            //     let right = shift.range.end..range.end;

            //     queue.push_back(left);
            //     queue.push_back(right);

            //     res.push(shift_by(shift.range, shift.by));
            // } else if shift.range.end < range.start || shift.range.start > range.end {
            //     continue;
            // }
        }

        // No shift applied, push range as-is
        res.push(range.clone());
    }
    res
}

impl Challenge for Part {
    fn input() -> &'static str {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn day(&self) -> u8 {
        5
    }

    fn part(&self) -> u8 {
        2
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "46"
        }

        #[cfg(not(debug_assertions))]
        {
            "60568880"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input();
        let (seeds, input) = input.split_once('\n').unwrap();
        let seeds = seeds.strip_prefix("seeds:").unwrap();

        let mut values: Vec<Range<usize>> = seeds
            .split_ascii_whitespace()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                let (start, length) = chunk.collect_tuple().unwrap();
                let start: usize = start.parse().unwrap();
                let length: usize = length.parse().unwrap();

                start..start + length
            })
            .collect();

        let maps = input.split("\n\n");

        for map in maps {
            let (header, map) = map.trim_start().split_once('\n').unwrap();
            let (_from, _to) = header
                .strip_suffix(" map:")
                .unwrap()
                .split_once("-to-")
                .unwrap();

            let mut shifts = Vec::new();
            for shift in map.lines() {
                let (destination, source, by) = shift.split_whitespace().collect_tuple().unwrap();

                let destination: usize = destination.parse().unwrap();
                let source: usize = source.parse().unwrap();
                let width: usize = by.parse().unwrap();

                let shift = Shift {
                    range: source..source + width,
                    by: (destination - source) as isize,
                };

                shifts.push(shift);
            }

            values = apply(values, shifts);
        }

        values.iter().map(|r| r.start).min().unwrap().to_string()
    }
}
