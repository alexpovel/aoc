use smallvec::SmallVec;

use crate::Challenge;

/// --- Part Two ---
///
/// The galaxies are much older (and thus much farther apart) than the researcher
/// initially estimated.
///
/// Now, instead of the expansion you did before, make each empty row or column one
/// million times larger. That is, each empty row should be replaced with 1000000 empty
/// rows, and each empty column should be replaced with 1000000 empty columns.
///
/// (In the example above, if each empty row or column were merely 10 times larger, the
/// sum of the shortest paths between every pair of galaxies would be 1030. If each
/// empty row or column were merely 100 times larger, the sum of the shortest paths
/// between every pair of galaxies would be 8410. However, your universe will need to
/// expand far beyond these values.)
///
/// Starting with the same initial image, expand the universe according to these new
/// rules, then find the length of the shortest path between every pair of galaxies.
/// What is the sum of these lengths?
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
        let input = Self::input();

        // `SmallVec` is approx. 10% faster (110µs -> ~95µs)
        let mut coords = SmallVec::<[(usize, usize); 1024]>::new();

        let mut empty_cols = Vec::from_iter(0..input.lines().next().unwrap().len());

        // Single row is *replaced* by whatever expansion is at hand, so subtract 1.
        const EXPANSION: usize = if cfg!(debug_assertions) {
            100 - 1
        } else {
            1_000_000 - 1
        };

        let mut row_offset = 0;
        for (i, line) in input.lines().enumerate() {
            let mut empty = true;

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    coords.push((i + row_offset, j));

                    empty_cols.retain(|&x| x != j);
                    empty = false;
                }
            }

            // Branchless makes no measurable difference here
            row_offset += empty as usize * EXPANSION;
        }

        // Expand. Cannot do this until now, until empty cols aren't known until now.
        for empty_col in empty_cols.iter().rev() {
            coords.iter_mut().for_each(|(_, j)| {
                if *j > *empty_col {
                    *j += EXPANSION;
                }
            });
        }

        let mut n = 0;

        // `combinations_with_replacement(2)` is massively slower (factor 10), probably
        // because it clones.
        (0..coords.len()).for_each(|left_index| {
            (left_index + 1..coords.len()).for_each(|right_index| {
                let left = coords[left_index];
                let right = coords[right_index];

                let max_i = left.0.max(right.0);
                let min_i = left.0.min(right.0);
                let max_j = left.1.max(right.1);
                let min_j = left.1.min(right.1);

                let distance = max_i - min_i + max_j - min_j;

                n += distance;
            })
        });

        n.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "8410"
        }

        #[cfg(not(debug_assertions))]
        {
            "746207878188"
        }
    }

    fn day(&self) -> u8 {
        11
    }

    fn part(&self) -> u8 {
        2
    }
}
