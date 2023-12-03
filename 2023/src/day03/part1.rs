use crate::iter::CenteredWindowExt;
use crate::Challenge;

/// --- Day 3: Gear Ratios ---
///
/// You and the Elf eventually reach a gondola lift station; he says the gondola lift
/// will take you up to the water source, but this is as far as he can bring you. You go
/// inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're
/// not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
/// "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll
/// still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but
/// nobody can figure out which one. If you can add up all the part numbers in the
/// engine schematic, it should be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the
/// engine. There are lots of numbers and symbols you don't really understand, but
/// apparently any number adjacent to a symbol, even diagonally, is a "part number" and
/// should be included in your sum. (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
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
/// In this schematic, two numbers are not part numbers because they are not adjacent to
/// a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a
/// symbol and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the
/// part numbers in the engine schematic?
pub struct Part {}

pub const BASE: u32 = 10;

#[derive(Debug, Clone, Copy)]
enum Number {
    Valid(u32),
    Invalid(u32),
}

impl Default for Number {
    fn default() -> Self {
        Self::Invalid(0)
    }
}

impl Number {
    fn value(&self) -> u32 {
        match self {
            Self::Valid(value) => *value,
            Self::Invalid(value) => *value,
        }
    }

    fn enlarge(&mut self, other: u32) -> &mut Self {
        match self {
            Self::Valid(value) => {
                *value *= BASE;
                *value += other;
            }
            Self::Invalid(value) => {
                *value *= BASE;
                *value += other;
            }
        }

        self
    }

    fn transition_from_neighbors(
        &mut self,
        up: Option<&str>,
        down: Option<&str>,
        left: Option<char>,
        right: Option<char>,
    ) -> &mut Self {
        match self {
            Self::Valid(_) => self,
            Self::Invalid(_) => {
                let is_symbol = |char: char| !char.is_ascii_digit() && char != '.';

                for snippet in &[up, down] {
                    match snippet {
                        Some(line) => {
                            for char in line.chars() {
                                if is_symbol(char) {
                                    *self = Self::Valid(self.value());
                                    return self;
                                }
                            }
                        }
                        None => {}
                    }
                }

                for char in &[left, right] {
                    match char {
                        Some(char) => {
                            if is_symbol(*char) {
                                *self = Self::Valid(self.value());
                                return self;
                            }
                        }
                        None => {}
                    }
                }

                self
            }
        }
    }
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
        3
    }

    fn part(&self) -> u8 {
        1
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "4361"
        }

        #[cfg(not(debug_assertions))]
        {
            "512794"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();

        let mut sum = 0;

        // This is the state: are we currently processing a number?
        let mut number = None;

        for (prev_line, current_line, next_line) in input.centered_window() {
            let length = current_line.len();

            for (i_char, (prev_char, curr_char, next_char)) in
                current_line.chars().centered_window().enumerate()
            {
                let start = i_char.saturating_sub(1);
                let end = (i_char + 1).min(length - 1);
                let range = start..=end;

                let up = || prev_line.map(|line| &line[range.clone()]);
                let down = || next_line.map(|line| &line[range.clone()]);

                match (number, curr_char.to_digit(BASE)) {
                    (None, None) => continue,
                    (None, Some(digit)) => {
                        let mut n = Number::default();
                        n.enlarge(digit);
                        n.transition_from_neighbors(up(), down(), prev_char, next_char);
                        number = Some(n);
                    }
                    (Some(mut n), Some(digit)) => {
                        n.enlarge(digit);
                        n.transition_from_neighbors(up(), down(), prev_char, next_char);

                        // Not sure why this is necessary; aren't we mutating in place?
                        number = Some(n);
                    }
                    (Some(n), None) => {
                        match n {
                            Number::Valid(value) => {
                                sum += value;
                            }
                            Number::Invalid(_) => {}
                        };

                        number = None;
                    }
                }
            }
        }

        format!("{}", sum)
    }
}
