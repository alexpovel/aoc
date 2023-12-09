use crate::Challenge;

/// --- Part Two ---
///
/// Of course, it would be nice to have even more history included in your report.
/// Surely it's safe to just extrapolate backwards as well, right?
///
/// For each history, repeat the process of finding differences until the sequence of
/// differences is entirely zero. Then, rather than adding a zero to the end and filling
/// in the next values of each previous sequence, you should instead add a zero to the
/// beginning of your sequence of zeroes, then fill in new first values for each
/// previous sequence.
///
/// In particular, here is what the third example history looks like when extrapolating
/// back in time:
///
/// ```text
/// 5  10  13  16  21  30  45
///   5   3   3   5   9  15
///    -2   0   2   4   6
///       2   2   2   2
///         0   0   0
/// ```
///
/// Adding the new values on the left side of each sequence from bottom to top
/// eventually reveals the new left-most history value: 5.
///
/// Doing this for the remaining example data above results in previous values of -3 for
/// the first history and 0 for the second history. Adding all three new values together
/// produces 2.
///
/// Analyze your OASIS report again, this time extrapolating the previous value for each
/// history. What is the sum of these extrapolated values?
pub struct Part {}

impl Challenge for Part {
    fn input() -> &'static str {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample1.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();

        // Idea: reuse a *single* buffer for everything. Fill it initially, then keep
        // writing all deltas flushed left-aligned. They'll always fit, as they're one
        // less than the previous line. They overwrite the *previous* deltas, but those
        // aren't needed anymore: only the *last* item is kept; that's needed for later
        // aggregation.
        let mut buffer = Vec::new();
        let mut total = 0;

        for line in input {
            buffer.clear();

            for number in line.split_ascii_whitespace() {
                let number: i32 = number.parse().unwrap();

                buffer.push(number);
            }

            buffer.reverse(); // ONLY CHANGE COMPARED TO PART 1

            // Going "deeper" corresponds to the lower levels in the example's
            // triangles.
            for depth in 1..buffer.len() {
                let mut all_zero = true;

                // "Mutable windows" isn't really a thing (?), so go old-school.
                for i in 0..buffer.len() - depth {
                    let a = buffer[i];
                    let b = buffer[i + 1];

                    let diff = b - a;

                    all_zero &= diff == 0;

                    buffer[i] = diff;
                }

                if all_zero {
                    // Breaking is not necessary for correct answers, but roughly 5%
                    // faster.
                    break;
                }
            }

            // For a sample like (see task description):
            //
            // ```text
            // 10  13  16  21  30  45
            //    3   3   5   9  15
            //    0   2   4   6
            //      2   2   2
            //        0   0
            // ```
            //
            // the buffer will look like:
            //
            // ```text
            // [ 0, 0, 2, 6, 15, 45 ]
            // ```
            //
            // so the "array of deltas/local slopes", whose sum "left to right" gives
            // the above level's next value.
            let next: i32 = buffer.iter().sum();
            total += next;
        }

        total.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "2"
        }

        #[cfg(not(debug_assertions))]
        {
            "884"
        }
    }

    fn day(&self) -> u8 {
        9
    }

    fn part(&self) -> u8 {
        2
    }
}
