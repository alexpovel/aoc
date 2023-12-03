use crate::{day03::part1::BASE, iter::CenteredWindowExt, Challenge};
use rangetools::Rangetools;
use regex::Regex;

/// --- Part Two ---
///
/// The engineer finds the missing part and installs it in the engine! As the engine
/// springs to life, you jump in the closest gondola, finally ready to ascend to the
/// water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong?
/// Fortunately, the gondola has a phone labeled "help", so you pick it up and the
/// engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window.
/// There stands the engineer, holding a phone in one hand and waving with the other.
/// You're going so slowly that you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong. A
/// gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is
/// the result of multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up so that
/// the engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
///
/// ```text
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
///
/// In this schematic, there are two gears. The first is in the top left; it has part
/// numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower
/// right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is
/// only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub struct Part {}

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
        3
    }

    fn part(&self) -> u8 {
        2
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "467835"
        }

        #[cfg(not(debug_assertions))]
        {
            "67779080"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();

        // Finally giving up and using a regex... wanted to try stdlib-only. This
        // compilation takes about 100 µs, doubling the runtime of this solution.
        let pattern = Regex::new(r"\d+").unwrap();

        let mut sum: u64 = 0;

        for (_i_line, (prev_line, curr_line, next_line)) in input.centered_window().enumerate() {
            let length = curr_line.len();

            'chars: for (i_char, (prev_char, curr_char, next_char)) in
                curr_line.chars().centered_window().enumerate()
            {
                if curr_char != '*' {
                    continue;
                }

                let start = i_char.saturating_sub(1);
                let end = (i_char + 1).min(length - 1);
                let range = start..=end;

                let mut gear_ratio = 1;
                let mut n = 0;
                const MAX_NEIGHBORS: u32 = 2;

                if let Some(char) = prev_char {
                    let mut number = 0;
                    if char.is_ascii_digit() {
                        for (i, c) in curr_line[..i_char]
                            .chars()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .enumerate()
                        {
                            number += c.to_digit(BASE).unwrap() * BASE.pow(i as u32);
                        }

                        n += 1;
                        gear_ratio *= number;
                    }
                }

                if let Some(char) = next_char {
                    if char.is_ascii_digit() {
                        let number: String = curr_line[i_char + 1..]
                            .chars()
                            .take_while(|c| c.is_ascii_digit())
                            .collect();

                        n += 1;
                        gear_ratio *= number.parse::<u32>().unwrap();
                    }
                }

                for line in [prev_line, next_line].iter().flatten() {
                    for match_ in pattern.find_iter(line) {
                        if range.clone().intersects(match_.range()) {
                            n += 1;

                            if n > MAX_NEIGHBORS {
                                continue 'chars;
                            }

                            gear_ratio *= match_.as_str().parse::<u32>().unwrap();
                        }
                    }
                }

                if n == MAX_NEIGHBORS {
                    sum += gear_ratio as u64;
                }
            }
        }

        format!("{}", sum)
    }
}
