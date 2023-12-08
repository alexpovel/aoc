use crate::Challenge;
use std::{sync::mpsc::channel, thread};

/// --- Part Two ---
///
/// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You
/// had the camel follow the instructions, but you've barely left your starting
/// position. It's going to take significantly more steps to escape!
///
/// What if the map isn't for people - what if the map is for ghosts? Are ghosts even
/// bound by the laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious fact:
/// the number of nodes with names ending in A is equal to the number ending in Z! If
/// you were a ghost, you'd probably just start at every node that ends with A and
/// follow all of the paths at the same time until they all simultaneously end up at
/// nodes that end with Z.
///
/// For example:
///
/// ```text
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
/// ```
///
/// Here, there are two starting nodes, 11A and 22A (because they both end with A). As
/// you follow each left/right instruction, use that instruction to simultaneously
/// navigate away from both nodes you're currently on. Repeat this process until all of
/// the nodes you're currently on end with Z. (If only some of the nodes you're on end
/// with Z, they act like any other node and you continue as normal.) In this example,
/// you would proceed as follows:
///
/// - Step 0: You are at 11A and 22A.
/// - Step 1: You choose all of the left paths, leading you to 11B and 22B.
/// - Step 2: You choose all of the right paths, leading you to 11Z and 22C.
/// - Step 3: You choose all of the left paths, leading you to 11B and 22Z.
/// - Step 4: You choose all of the right paths, leading you to 11Z and 22B.
/// - Step 5: You choose all of the left paths, leading you to 11B and 22C.
/// - Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
///
/// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
///
/// Simultaneously start on every node that ends with A. How many steps does it take
/// before you're only on nodes that end with Z?
pub struct Part {}

impl Challenge for Part {
    fn input() -> &'static str {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample3.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let input = Self::input();
        let (directions, input) = input.split_once("\n\n").unwrap();

        // `evmap` and vanilla threads doesn't lead to performance increases compared to
        // standard `HashMap` with `rayon` and its `par_iter` *in this case*, but a fun
        // exercise... In total, parallelization (with either approach) drops execution
        // time from 1.8ms to 1.2ms.
        let (map_rh, mut map_wh) = evmap::new();

        let mut start_nodes = Vec::new();
        for line in input.lines() {
            let (node, targets) = line.split_once(" = ").unwrap();

            let (left, right) = targets
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            if node.ends_with('A') {
                start_nodes.push(node);
            }

            map_wh.insert(node, (left, right));
        }

        map_wh.refresh(); // Flush writes

        let (sender, receiver) = channel();

        let handles = start_nodes
            .into_iter()
            .map(|mut node| {
                let rh = map_rh.clone();
                let s = sender.clone();

                thread::spawn(
                    // This walks every start node to its first end, aka `Z` node.
                    // That's all!
                    move || {
                        for (i, direction) in directions.chars().cycle().enumerate() {
                            let read_guard = rh.get_one(node).unwrap();
                            let (left, right) = (read_guard.0, read_guard.1);

                            let next = match direction {
                                'L' => left,
                                'R' => right,
                                _ => unreachable!(),
                            };

                            if next.ends_with('Z') {
                                s.send(i + 1).unwrap();
                                break;
                            }

                            node = next;
                        }
                    },
                )
            })
            .collect::<Vec<_>>();

        drop(sender); // Allow receiver loop to finish

        for handle in handles {
            handle.join().unwrap();
        }

        // Why does LCM even work? The *general* case for this problem is incredibly
        // complex. It can only be solved trivially, aka with LCM, as the following
        // observations about the input data hold true (and are exploited). I probably
        // even forgot some...
        //
        // - path length % length of instructions == 0, so when revisiting a Z node, the
        //   SAME direction as before is taken *again*, otherwise a single Z node is
        //   part of multiple loops, and a single Z node is exited *differently*
        //   depending on instruction step, leading to wildly diverging paths.
        // - single Z in *entire* path, including initial and loop
        // - all paths, from each start node to its end Z node, including all loops, are
        //   *disjoint* (they do not cross)
        // - path from initial A to the (unique) Z, then path back to that same Z are
        //   equidistant, such that LCM "just works", no offsets needed
        // - no A node has incoming edges
        //
        // Other observations:
        //
        // - paths can loop *before* ever encountering their first `Z` node
        let mut least_common_multiple = 1;
        for distance in receiver {
            least_common_multiple = num::integer::lcm(least_common_multiple, distance);
        }

        least_common_multiple.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "6"
        }

        #[cfg(not(debug_assertions))]
        {
            "18215611419223"
        }
    }

    fn day(&self) -> u8 {
        8
    }

    fn part(&self) -> u8 {
        2
    }
}
