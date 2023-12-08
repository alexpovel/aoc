use std::str::FromStr;

use crate::Challenge;

/// --- Part Two ---
///
/// To make things a little more interesting, the Elf introduces one additional rule.
/// Now, J cards are jokers - wildcards that can act like whatever card would make the
/// hand the strongest type possible.
///
/// To balance this, J cards are now the weakest individual cards, weaker even than 2.
/// The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
///
/// J cards can pretend to be whatever card is best for the purpose of determining hand
/// type; for example, QJJQ2 is now considered four of a kind. However, for the purpose
/// of breaking ties between two hands of the same type, J is always treated as J, not
/// the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than
/// Q.
///
/// Now, the above example goes very differently:
///
/// ```text
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// - 32T3K is still the only one pair; it doesn't contain any jokers, so its strength
///   doesn't increase.
/// - KK677 is now the only two pair, making it the second-weakest hand.
/// - T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets
///   rank 4, and KTJJT gets rank 5.
///
/// With the new joker rule, the total winnings in this example are 5905.
///
/// Using the new joker rule, find the rank of every hand in your set. What are the new
/// total winnings?
pub struct Part {}

/// Deriving `PartialOrd`: first is *least* significant!
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "K" => Ok(Self::K),
            "Q" => Ok(Self::Q),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            "J" => Ok(Self::J),
            _ => unreachable!("invalid card"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cards([Card; 5]);

impl FromStr for Cards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::A; 5];

        for (i, card) in s.chars().enumerate() {
            cards[i] = Card::from_str(&card.to_string()).unwrap();
        }

        Ok(Self(cards))
    }
}

/// Deriving `PartialOrd`: first is *least* significant!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard(Cards),
    OnePair(Cards),
    TwoPair(Cards),
    ThreeOfAKind(Cards),
    FullHouse(Cards),
    FourOfAKind(Cards),
    FiveOfAKind(Cards),
}

impl From<Cards> for Hand {
    fn from(cards: Cards) -> Self {
        let sorted = {
            let mut copy = cards.0.to_owned();
            copy.sort_unstable();
            copy
        };

        match (
            sorted[0] == sorted[1],
            sorted[1] == sorted[2],
            sorted[2] == sorted[3],
            sorted[3] == sorted[4],
        ) {
            (true, true, true, true) => Self::FiveOfAKind(cards),

            (true, true, true, false) => match sorted[3] {
                Card::J => Self::FiveOfAKind(cards),
                _ => Self::FourOfAKind(cards),
            },

            (false, true, true, true) => match sorted[0] {
                Card::J => Self::FiveOfAKind(cards),
                _ => Self::FourOfAKind(cards),
            },

            (true, false, true, true) => match sorted[1] {
                Card::J => Self::FiveOfAKind(cards),
                _ => Self::FullHouse(cards),
            },

            (true, true, false, true) => match sorted[2] {
                Card::J => Self::FiveOfAKind(cards),
                _ => Self::FullHouse(cards),
            },

            (false, true, true, false) | (false, false, true, true) => match sorted[0] {
                Card::J => Self::FourOfAKind(cards),
                _ => Self::ThreeOfAKind(cards),
            },

            (true, true, false, false) => match sorted[2] {
                Card::J => Self::FourOfAKind(cards),
                _ => Self::ThreeOfAKind(cards),
            },

            (true, false, false, true) | (true, false, true, false) => match sorted[1] {
                Card::J => Self::FourOfAKind(cards),
                _ => Self::TwoPair(cards),
            },

            (false, true, false, true) => match sorted[0] {
                Card::J => Self::FullHouse(cards),
                _ => Self::TwoPair(cards),
            },

            (true, false, false, false) => match sorted[1] {
                Card::J => Self::ThreeOfAKind(cards),
                _ => Self::OnePair(cards),
            },

            (false, true, false, false)
            | (false, false, true, false)
            | (false, false, false, true) => match sorted[0] {
                Card::J => Self::ThreeOfAKind(cards),
                _ => Self::OnePair(cards),
            },

            (false, false, false, false) => match sorted[0] {
                Card::J => Self::OnePair(cards),
                _ => Self::HighCard(cards),
            },
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
        7
    }

    fn part(&self) -> u8 {
        2
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "5905"
        }

        #[cfg(not(debug_assertions))]
        {
            "251224870"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();

        let mut hands = Vec::new();
        for line in input {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: usize = bid.parse().unwrap();
            let cards = Cards::from_str(hand).unwrap();
            let hand = Hand::from(cards);

            hands.push((hand, bid));
        }

        // Unstable found to be c. 10% faster, as it's allocation-free, in-place. Input
        // contains no dupes (trust me bro).
        hands.sort_unstable_by_key(|&(h, _)| h);

        let mut total = 0;
        for (i, (_, bid)) in hands.iter().enumerate() {
            let rank = i + 1;
            total += bid * rank;
        }

        total.to_string()
    }
}
