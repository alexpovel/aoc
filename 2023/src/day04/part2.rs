use crate::Challenge;

/// --- Part Two ---
///
/// Just as you're about to report your findings to the Elf, one of you realizes that
/// the rules have actually been printed on the back of every card this whole time.
///
/// There's no such thing as "points". Instead, scratchcards only cause you to win more
/// scratchcards equal to the number of winning numbers you have.
///
/// Specifically, you win copies of the scratchcards below the winning card equal to the
/// number of matches. So, if card 10 were to have 5 matching numbers, you would win one
/// copy each of cards 11, 12, 13, 14, and 15.
///
/// Copies of scratchcards are scored like normal scratchcards and have the same card
/// number as the card they copied. So, if you win a copy of card 10 and it has 5
/// matching numbers, it would then win a copy of the same cards that the original card
/// 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies
/// cause you to win any more cards. (Cards will never make you copy a card past the end
/// of the table.)
///
/// This time, the above example goes differently:
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
/// - Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
/// - Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
/// - Your copy of card 2 also wins one copy each of cards 3 and 4.
/// - Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
/// - Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
/// - Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
/// - Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
///
/// Once all of the originals and copies have been processed, you end up with 1 instance
/// of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14
/// instances of card 5, and 1 instance of card 6. In total, this example pile of
/// scratchcards causes you to ultimately have 30 scratchcards!
///
/// Process all of the original and copied scratchcards until no more scratchcards are
/// won. Including the original set of scratchcards, how many total scratchcards do you
/// end up with?
pub struct Part {}

impl Challenge for Part {
    fn input() -> &'static str
    where
        Self: Sized,
    {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();
        let mut n_cards = [1; 4096]; // 🤔
        let mut sum = 0;

        for (i_card, line) in input.enumerate() {
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

            let n_current = n_cards[i_card];
            for k in 1..(n + 1) {
                n_cards[i_card + k] += n_current;
            }

            sum += n_current;
        }

        sum.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "30"
        }

        #[cfg(not(debug_assertions))]
        {
            "6227972"
        }
    }

    fn day(&self) -> u8 {
        4
    }

    fn part(&self) -> u8 {
        2
    }
}
