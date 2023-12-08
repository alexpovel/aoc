use crate::Challenge;
use ahash::AHashMap;

/// --- Day 8: Haunted Wasteland ---
///
/// You're still riding a camel across Desert Island when you spot a sandstorm quickly
/// approaching. When you turn to warn the Elf, she disappears before your eyes! To be
/// fair, she had just finished warning you about ghosts a few minutes ago.
///
/// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents
/// (your puzzle input) about how to navigate the desert. At least, you're pretty sure
/// that's what they are; one of the documents contains a list of left/right
/// instructions, and the rest of the documents seem to describe some kind of network of
/// labeled nodes.
///
/// It seems like you're meant to use the left/right instructions to navigate the
/// network. Perhaps if you have the camel follow the same instructions, you can escape
/// the haunted wasteland!
///
/// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like
/// AAA is where you are now, and you have to follow the left/right instructions until
/// you reach ZZZ.
///
/// This format defines each node of the network individually. For example:
///
/// ```text
/// RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting with AAA, you need to look up the next element based on the next left/right
/// instruction in your input. In this example, start with AAA and go right (R) by
/// choosing the right element of AAA, CCC. Then, L means to choose the left element of
/// CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.
///
/// Of course, you might not find ZZZ right away. If you run out of left/right
/// instructions, repeat the whole sequence of instructions as necessary: RL really
/// means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6
/// steps to reach ZZZ:
///
/// ```text
/// LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting at AAA, follow the left/right instructions. How many steps are required to
/// reach ZZZ?
pub struct Part {}

pub(super) fn hash(string: &str) -> u32 {
    let mut bytes = [0; 4];
    bytes[1..].copy_from_slice(string.as_bytes());
    u32::from_le_bytes(bytes)
}

impl Challenge for Part {
    fn input() -> &'static str {
        #[cfg(debug_assertions)]
        {
            // include_str!("input/sample1.txt")
            include_str!("input/sample2.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let input = Self::input();
        let (directions, input) = input.split_once("\n\n").unwrap();
        let directions = directions.chars().cycle();

        // Default performance (vanilla `HashMap` and inserting `&str`) found to be
        // 350µs. Using vanilla `ahash` was 260µs. Collecting all lines, then sorting,
        // then binary searching, was found to sit at 550µs. Writing the most simple
        // hash function (`&str` -> `u32`) sits at 210µs. *Combining* `ahash` and using
        // `u32` keys from the hash function sits at 190µs.
        let mut map = AHashMap::new();

        for line in input.lines() {
            let (node, targets) = line.split_once(" = ").unwrap();

            let (left, right) = targets
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            map.insert(hash(node), (left, right));
        }

        const START: &str = "AAA";
        const TARGET: &str = "ZZZ";

        let mut n = 0;
        let mut node = START;

        for direction in directions {
            if node == TARGET {
                break;
            }

            let (left, right) = map.get(&hash(node)).unwrap();

            match direction {
                'L' => node = left,
                'R' => node = right,
                _ => unreachable!(),
            }

            n += 1;
        }

        n.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "6"
        }

        #[cfg(not(debug_assertions))]
        {
            "12361"
        }
    }

    fn day(&self) -> u8 {
        8
    }

    fn part(&self) -> u8 {
        1
    }
}
