use itertools::Itertools;

use crate::Challenge;

/// --- Day 10: Pipe Maze ---
///
/// You use the hang glider to ride the hot air from Desert Island all the way up to the
/// floating metal island. This island is surprisingly cold and there definitely aren't
/// any thermals to glide on, so you leave your hang glider behind.
///
/// You wander around for a while, but you don't find any people or animals. However,
/// you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly
/// consistent direction; maybe you can find someone at the hot springs and ask them
/// where the desert-machine parts are made.
///
/// The landscape here is alien; even the flowers and trees are made of metal. As you
/// stop to admire some metal grass, you notice something metallic scurry away in your
/// peripheral vision and jump into a big pipe! It didn't look like any animal you've
/// ever seen; if you want a better look, you'll need to get ahead of it.
///
/// Scanning the area, you discover that the entire field you're standing on is densely
/// packed with pipes; it was hard to tell at first because they're the same metallic
/// silver color as the "ground". You make a quick sketch of all of the surface pipes
/// you can see (your puzzle input).
///
/// The pipes are arranged in a two-dimensional grid of tiles:
///
/// - | is a vertical pipe connecting north and south.
/// - - is a horizontal pipe connecting east and west.
/// - L is a 90-degree bend connecting north and east.
/// - J is a 90-degree bend connecting north and west.
/// - 7 is a 90-degree bend connecting south and west.
/// - F is a 90-degree bend connecting south and east.
/// - . is ground; there is no pipe in this tile.
/// - S is the starting position of the animal; there is a pipe on this tile, but your
///   sketch doesn't show what shape the pipe has.
///
/// Based on the acoustics of the animal's scurrying, you're confident the pipe that
/// contains the animal is one large, continuous loop.
///
/// For example, here is a square loop of pipe:
///
/// ```text
/// .....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// If the animal had entered this loop in the northwest corner, the sketch would
/// instead look like this:
///
/// ```text
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// In the above diagram, the S tile is still a 90-degree F bend: you can tell because
/// of how the adjacent pipes connect to it.
///
/// Unfortunately, there are also many pipes that aren't connected to the loop! This
/// sketch shows the same loop as above:
///
/// ```text
/// -L|F7
/// 7S-7|
/// L|7||
/// -L-J|
/// L|-JF
/// ```
///
/// In the above diagram, you can still figure out which pipes form the main loop:
/// they're the ones connected to S, pipes those pipes connect to, pipes those pipes
/// connect to, and so on. Every pipe in the main loop connects to its two neighbors
/// (including S, which will have exactly two pipes connecting to it, and which is
/// assumed to connect back to those two pipes).
///
/// Here is a sketch that contains a slightly more complex main loop:
///
/// ```text
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
///
/// ```text
/// 7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ
/// ```
///
/// If you want to get out ahead of the animal, you should find the tile in the loop
/// that is farthest from the starting position. Because the animal is in the pipe, it
/// doesn't make sense to measure this by direct distance. Instead, you need to find the
/// tile that would take the longest number of steps along the loop to reach from the
/// starting point - regardless of which way around the loop the animal went.
///
/// In the first example with the square loop:
///
/// ```text
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// You can count the distance each tile in the loop is from the starting point like
/// this:
///
/// ```text
/// .....
/// .012.
/// .1.3.
/// .234.
/// .....
/// ```
///
/// In this example, the farthest point from the start is 4 steps away.
///
/// Here's the more complex loop again:
///
/// ```text
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here are the distances for each tile on that loop:
///
/// ```text
/// ..45.
/// .236.
/// 01.78
/// 14567
/// 23...
/// ```
///
/// Find the single giant loop starting at S. How many steps along the loop does it take
/// to get from the starting position to the point farthest from the starting position?
pub struct Part {}

impl Challenge for Part {
    fn input() -> &'static str {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample1.txt")
            // include_str!("input/sample2.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let map = Self::input()
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let start = map
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, &c)| if c == 'S' { Some((i, j)) } else { None })
            })
            .unwrap();

        let mut pos = start;
        let mut prev_pos = None;

        let mut n = 0;

        let mut possible = Vec::with_capacity(4);

        loop {
            let element = map[pos.0][pos.1];

            if element == 'S' && prev_pos.is_some() {
                break;
            }

            possible.clear();

            if pos.1 > 0 {
                let west = map[pos.0].get(pos.1 - 1);
                if let Some(e) = west {
                    if let ('-' | 'L' | 'F' | 'S', '-' | 'J' | '7' | 'S') = (e, element) {
                        possible.push((pos.0, pos.1 - 1));
                    }
                }
            }

            let east = map[pos.0].get(pos.1 + 1);
            if let Some(e) = east {
                if let ('-' | 'J' | '7' | 'S', '-' | 'F' | 'L' | 'S') = (e, element) {
                    possible.push((pos.0, pos.1 + 1));
                }
            }

            if pos.0 > 0 {
                let north_line = map.get(pos.0 - 1);
                if let Some(north) = north_line {
                    if let ('|' | 'F' | '7' | 'S', '|' | 'L' | 'J' | 'S') = (north[pos.1], element)
                    {
                        possible.push((pos.0 - 1, pos.1));
                    }
                }
            }

            let south_line = map.get(pos.0 + 1);
            if let Some(south) = south_line {
                if let ('|' | 'L' | 'J' | 'S', '|' | '7' | 'F' | 'S') = (south[pos.1], element) {
                    possible.push((pos.0 + 1, pos.1));
                }
            }

            n += 1;

            let next = if let Some(pp) = prev_pos {
                possible.iter().find(|&&pos| pos != pp).unwrap()
            } else {
                // We're at `S`: two are possible, and no previous available. Just pick
                // one.
                possible.first().unwrap()
            };

            prev_pos = Some(pos);
            pos = *next;
        }

        (n / 2).to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "4"
            // "8"
        }

        #[cfg(not(debug_assertions))]
        {
            "6951"
        }
    }

    fn day(&self) -> u8 {
        10
    }

    fn part(&self) -> u8 {
        1
    }
}
