use smallvec::SmallVec;

use crate::Challenge;

/// --- Day 11: Cosmic Expansion ---
///
/// You continue following signs for "Hot Springs" and eventually come across an
/// observatory. The Elf within turns out to be a researcher studying cosmic expansion
/// using the giant telescope here.
///
/// He doesn't know anything about the missing machine parts; he's only visiting for
/// this research project. However, he confirms that the hot springs are the
/// next-closest area likely to have people; he'll even take you straight there once
/// he's done with today's observation analysis.
///
/// Maybe you can help him with the analysis to speed things up?
///
/// The researcher has collected a bunch of data and compiled the data into a single
/// giant image (your puzzle input). The image includes empty space (.) and galaxies
/// (#). For example:
///
/// ```text
/// ...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....
/// ```
///
/// The researcher is trying to figure out the sum of the lengths of the shortest path
/// between every pair of galaxies. However, there's a catch: the universe expanded in
/// the time it took the light from those galaxies to reach the observatory.
///
/// Due to something involving gravitational effects, only some space expands. In fact,
/// the result is that any rows or columns that contain no galaxies should all actually
/// be twice as big.
///
/// In the above example, three columns and two rows contain no galaxies:
///
/// ```text
///    v  v  v
///  ...#......
///  .......#..
///  #.........
/// >..........<
///  ......#...
///  .#........
///  .........#
/// >..........<
///  .......#..
///  #...#.....
///    ^  ^  ^
/// ```
///
/// These rows and columns need to be twice as big; the result of cosmic expansion
/// therefore looks like this:
///
/// ```text
/// ....#........
/// .........#...
/// #............
/// .............
/// .............
/// ........#....
/// .#...........
/// ............#
/// .............
/// .............
/// .........#...
/// #....#.......
/// ```
///
/// Equipped with this expanded universe, the shortest path between every pair of
/// galaxies can be found. It can help to assign every galaxy a unique number:
///
/// ```text
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// ............6
/// .............
/// .............
/// .........7...
/// 8....9.......
/// ```
///
/// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the
/// pair doesn't matter. For each pair, find any shortest path between the two galaxies
/// using only steps that move up, down, left, or right exactly one . or # at a time.
/// (The shortest path between two galaxies is allowed to pass through another galaxy.)
///
/// For example, here is one of the shortest paths between galaxies 5 and 9:
///
/// ```text
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// .##.........6
/// ..##.........
/// ...##........
/// ....##...7...
/// 8....9.......
/// ```
///
/// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5
/// to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here
/// are some other example shortest path lengths:
///
/// - Between galaxy 1 and galaxy 7: 15
/// - Between galaxy 3 and galaxy 6: 17
/// - Between galaxy 8 and galaxy 9: 5
///
/// In this example, after expanding the universe, the sum of the shortest path between
/// all 36 pairs of galaxies is 374.
///
/// Expand the universe, then find the length of the shortest path between every pair of
/// galaxies. What is the sum of these lengths?
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

            row_offset += empty as usize;
        }

        // Expand. Cannot do this until now, until empty cols aren't known until now.
        for empty_col in empty_cols.iter().rev() {
            coords.iter_mut().for_each(|(_, j)| {
                if *j > *empty_col {
                    *j += 1;
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
            "374"
        }

        #[cfg(not(debug_assertions))]
        {
            "9370588"
        }
    }

    fn day(&self) -> u8 {
        11
    }

    fn part(&self) -> u8 {
        1
    }
}
