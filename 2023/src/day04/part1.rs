use crate::Challenge;

/// --- Day 4: Scratchcards ---
///
/// The gondola takes you up. Strangely, though, the ground doesn't seem to be coming
/// with you; you're not climbing a mountain. As the circle of Snow Island recedes below
/// you, an entire new landmass suddenly appears above you! The gondola carries you to
/// the surface of the new island and lurches into the station.
///
/// As you exit the gondola, the first thing you notice is that the air here is much
/// warmer than it was on Snow Island. It's also quite humid. Is this where the water
/// source is?
///
/// The next thing you notice is an Elf sitting on the floor across the station in what
/// seems to be a pile of colorful square cards.
///
/// "Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You ask
/// about water sources.
///
/// "I'm not sure; I just operate the gondola lift. That does sound like something we'd
/// have, though - this is Island Island, after all! I bet the gardener would know. He's
/// on a different island, though - er, the small kind surrounded by water, not the
/// floating kind. We really need to come up with a better naming scheme. Tell you what:
/// if you can help me with something quick, I'll let you borrow my boat and you can go
/// visit the gardener. I got all these scratchcards as a gift, but I can't figure out
/// what I've won."
///
/// The Elf leads you over to the pile of colorful cards. There, you discover dozens of
/// scratchcards, all with their opaque covering already scratched off. Picking one up,
/// it looks like each card has two lists of numbers separated by a vertical bar (|): a
/// list of winning numbers and then a list of numbers you have. You organize the
/// information into a table (your puzzle input).
///
/// As far as the Elf has been able to figure out, you have to figure out which of the
/// numbers you have appear in the list of winning numbers. The first match makes the
/// card worth one point and each match after the first doubles the point value of that
/// card.
///
/// For example:
///
/// ```
/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
/// ```
///
/// In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and
/// eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have,
/// four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8
/// points (1 for the first match, then doubled three times for each of the three
/// matches after the first).
///
///     Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
///     Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
///     Card 4 has one winning number (84), so it is worth 1 point.
///     Card 5 has no winning numbers, so it is worth no points.
///     Card 6 has no winning numbers, so it is worth no points.
///
/// So, in this example, the Elf's pile of scratchcards is worth 13 points.
///
/// Take a seat in the large pile of colorful cards. How many points are they worth in
/// total?
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
        4
    }

    fn part(&self) -> u8 {
        1
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "13"
        }

        #[cfg(not(debug_assertions))]
        {
            "26426"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();
        let mut sum = 0;

        for line in input {
            const N_WINNING_NUMBERS: usize = if cfg!(debug_assertions) { 5 } else { 10 };
            const N_DRAWN_NUMBERS: usize = if cfg!(debug_assertions) { 8 } else { 25 };

            let mut winning_numbers = [0; N_WINNING_NUMBERS];
            let mut drawn_numbers = [0; N_DRAWN_NUMBERS];

            static FIRST_INDENT: usize = if cfg!(debug_assertions) { 8 } else { 10 };
            static NUMBER_WIDTH: usize = 2;
            static STEP: usize = NUMBER_WIDTH + ' '.len_utf8();
            static SECOND_INDENT: usize = FIRST_INDENT + N_WINNING_NUMBERS * STEP + " | ".len() - 1;

            for (i, n) in winning_numbers.iter_mut().enumerate() {
                let start = FIRST_INDENT + i * STEP;
                let end = start + NUMBER_WIDTH;
                let l = line[start..end].trim_start();

                *n = l.parse().unwrap();
            }

            for (i, n) in drawn_numbers.iter_mut().enumerate() {
                let start = SECOND_INDENT + i * STEP;
                let end = start + NUMBER_WIDTH;
                let l = line[start..end].trim_start();

                *n = l.parse().unwrap();
            }

            let mut n = 0;
            // Quadratic complexity, but at low numbers (number of drawn cards is just
            // 25), probably more performant than set operations, especially since all
            // items are on the stack (hashes are on the heap). Untested hypothesis!
            for drawn_number in drawn_numbers.iter() {
                for winning_number in winning_numbers.iter() {
                    if drawn_number == winning_number {
                        n += 1;
                    }
                }
            }

            if n > 0 {
                sum += u32::pow(2, n - 1);
            }
        }

        sum.to_string()
    }
}
