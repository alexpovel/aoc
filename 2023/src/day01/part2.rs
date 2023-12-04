use crate::Challenge;

/// Your calculation isn't quite right. It looks like some of the digits are actually
/// spelled out with letters: one, two, three, four, five, six, seven, eight, and nine
/// also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last
/// digit on each line. For example:
///
/// ```
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding
/// these together produces 281.
///
/// What is the sum of all of the calibration values?
pub struct Part {}

enum ForwardState {
    Empty,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}

enum BackwardsState {
    Empty,
    /// One, Three, Five, Nine
    E,
    /// One, Nine
    En,
    /// Two
    O,
    /// Two
    Ow,
    /// Three
    Ee,
    /// Three
    Eer,
    /// Three
    Eerh,
    /// Four
    R,
    /// Four
    Ru,
    /// Four
    Ruo,
    /// Five
    Ev,
    /// Five
    Evi,
    /// Six
    X,
    /// Six
    Xi,
    /// Seven
    N,
    /// Seven
    Ne,
    /// Seven
    Nev,
    /// Seven
    Neve,
    /// Eight
    T,
    /// Eight
    Th,
    /// Eight
    Thg,
    /// Eight
    Thgi,
    /// Nine
    Eni,
}

impl Challenge for Part {
    fn input() -> &'static str
    where
        Self: Sized,
    {
        #[cfg(debug_assertions)]
        {
            include_str!("input/sample/2.txt")
        }

        #[cfg(not(debug_assertions))]
        {
            include_str!("input/1.txt")
        }
    }

    fn solve(&self) -> String {
        let input = Self::input().lines();

        let mut sum = 0;
        const BASE: u32 = 10;

        for line in input {
            let mut forward_state = ForwardState::Empty;
            let mut backward_state = BackwardsState::Empty;
            let mut digit = 0;

            for char in line.chars() {
                if char.is_ascii_digit() {
                    digit = char.to_digit(BASE).unwrap();
                    break;
                }

                // All I want for Christmas is compile-time regex support :( when going
                // for performance, building the regex at runtime is surprisingly
                // expensive.
                //
                // ⚠️ means: failure to match, see if any other track works. Would be
                // backtracking in regex.
                forward_state = match (forward_state, char) {
                    //
                    // Empty
                    (ForwardState::Empty, 'o') => ForwardState::O,
                    (ForwardState::Empty, 't') => ForwardState::T,
                    (ForwardState::Empty, 'f') => ForwardState::F,
                    (ForwardState::Empty, 's') => ForwardState::S,
                    (ForwardState::Empty, 'e') => ForwardState::E,
                    (ForwardState::Empty, 'n') => ForwardState::N,
                    //
                    // Single-letter
                    (ForwardState::O, 'n') => ForwardState::On,
                    (ForwardState::T, 'w') => ForwardState::Tw,
                    (ForwardState::T, 'h') => ForwardState::Th,
                    (ForwardState::F, 'o') => ForwardState::Fo,
                    (ForwardState::F, 'i') => ForwardState::Fi,
                    (ForwardState::S, 'i') => ForwardState::Si,
                    (ForwardState::S, 'e') => ForwardState::Se,
                    (ForwardState::E, 'i') => ForwardState::Ei,
                    (ForwardState::N, 'i') => ForwardState::Ni,
                    //
                    // Two-letter
                    (ForwardState::Tw, 'o') => {
                        digit = 2;
                        break;
                    }
                    (ForwardState::Th, 'r') => ForwardState::Thr,
                    (ForwardState::Fo, 'u') => ForwardState::Fou,
                    (ForwardState::Fo, 'n') => ForwardState::On, // ⚠️
                    (ForwardState::Fi, 'v') => ForwardState::Fiv,
                    (ForwardState::Si, 'x') => {
                        digit = 6;
                        break;
                    }
                    (ForwardState::Se, 'v') => ForwardState::Sev,
                    (ForwardState::Se, 'i') => ForwardState::Ei, // ⚠️
                    (ForwardState::Ei, 'g') => ForwardState::Eig,
                    (ForwardState::Ni, 'n') => ForwardState::Nin,
                    (ForwardState::On, 'e') => {
                        #[allow(clippy::identity_op)]
                        {
                            digit = 1;
                            break;
                        }
                    }
                    (ForwardState::On, 'i') => ForwardState::Ni, // ⚠️
                    //
                    // Three-letter
                    (ForwardState::Thr, 'e') => ForwardState::Thre,
                    (ForwardState::Sev, 'e') => ForwardState::Seve,
                    (ForwardState::Eig, 'h') => ForwardState::Eigh,
                    (ForwardState::Nin, 'i') => ForwardState::Ni, // ⚠️
                    (ForwardState::Nin, 'e') => {
                        digit = 9;
                        break;
                    }
                    (ForwardState::Fou, 'r') => {
                        digit = 4;
                        break;
                    }
                    (ForwardState::Fiv, 'e') => {
                        digit = 5;
                        break;
                    }
                    //
                    // Four-letter
                    (ForwardState::Thre, 'e') => {
                        digit = 3;
                        break;
                    }
                    (ForwardState::Seve, 'n') => {
                        digit = 7;
                        break;
                    }
                    (ForwardState::Eigh, 't') => {
                        digit = 8;
                        break;
                    }
                    //
                    // Reset
                    (_, 'o') => ForwardState::O,
                    (_, 't') => ForwardState::T,
                    (_, 'f') => ForwardState::F,
                    (_, 's') => ForwardState::S,
                    (_, 'e') => ForwardState::E,
                    (_, 'n') => ForwardState::N,
                    _ => ForwardState::Empty,
                }
            }

            assert!(digit != 0);
            digit *= BASE;

            for char in line.chars().rev() {
                if char.is_ascii_digit() {
                    digit += char.to_digit(BASE).unwrap();
                    break;
                }

                backward_state = match (backward_state, char) {
                    //
                    // Empty
                    (BackwardsState::Empty, 'e') => BackwardsState::E,
                    (BackwardsState::Empty, 'o') => BackwardsState::O,
                    (BackwardsState::Empty, 'r') => BackwardsState::R,
                    (BackwardsState::Empty, 'x') => BackwardsState::X,
                    (BackwardsState::Empty, 'n') => BackwardsState::N,
                    (BackwardsState::Empty, 't') => BackwardsState::T,
                    //
                    // Single-letter
                    (BackwardsState::E, 'n') => BackwardsState::En,
                    (BackwardsState::E, 'e') => BackwardsState::Ee,
                    (BackwardsState::E, 'v') => BackwardsState::Ev,
                    (BackwardsState::R, 'u') => BackwardsState::Ru,
                    (BackwardsState::X, 'i') => BackwardsState::Xi,
                    (BackwardsState::N, 'e') => BackwardsState::Ne,
                    (BackwardsState::T, 'h') => BackwardsState::Th,
                    (BackwardsState::O, 'w') => BackwardsState::Ow,
                    //
                    // Two-letter
                    (BackwardsState::En, 'i') => BackwardsState::Eni,
                    (BackwardsState::En, 'e') => BackwardsState::Ne, // ⚠️
                    (BackwardsState::Ee, 'r') => BackwardsState::Eer,
                    (BackwardsState::Ee, 'n') => BackwardsState::En, // ⚠️
                    (BackwardsState::Ee, 'e') => BackwardsState::Ee, // ⚠️
                    (BackwardsState::Ee, 'v') => BackwardsState::Ev, // ⚠️
                    (BackwardsState::Ev, 'i') => BackwardsState::Evi,
                    (BackwardsState::Th, 'g') => BackwardsState::Thg,
                    (BackwardsState::Ne, 'v') => BackwardsState::Nev,
                    (BackwardsState::Ne, 'n') => BackwardsState::En, // ⚠️
                    (BackwardsState::Ne, 'e') => BackwardsState::Ee, // ⚠️
                    (BackwardsState::Ru, 'o') => BackwardsState::Ruo,
                    (BackwardsState::En, 'o') => {
                        digit += 1;
                        break;
                    }
                    (BackwardsState::Ow, 't') => {
                        digit += 2;
                        break;
                    }
                    (BackwardsState::Xi, 's') => {
                        digit += 6;
                        break;
                    }
                    //
                    // Three-letter
                    (BackwardsState::Eer, 'h') => BackwardsState::Eerh,
                    (BackwardsState::Eer, 'u') => BackwardsState::Ru, // ⚠️
                    (BackwardsState::Thg, 'i') => BackwardsState::Thgi,
                    (BackwardsState::Nev, 'e') => BackwardsState::Neve,
                    (BackwardsState::Nev, 'i') => BackwardsState::Evi, // ⚠️
                    (BackwardsState::Ruo, 'f') => {
                        digit += 4;
                        break;
                    }
                    (BackwardsState::Evi, 'f') => {
                        digit += 5;
                        break;
                    }
                    (BackwardsState::Eni, 'n') => {
                        digit += 9;
                        break;
                    }
                    //
                    // Four-letter
                    (BackwardsState::Eerh, 't') => {
                        digit += 3;
                        break;
                    }
                    (BackwardsState::Thgi, 'e') => {
                        digit += 8;
                        break;
                    }
                    (BackwardsState::Neve, 's') => {
                        digit += 7;
                        break;
                    }
                    //
                    // Reset
                    (_, 'e') => BackwardsState::E,
                    (_, 'o') => BackwardsState::O,
                    (_, 'r') => BackwardsState::R,
                    (_, 'x') => BackwardsState::X,
                    (_, 'n') => BackwardsState::N,
                    (_, 't') => BackwardsState::T,
                    _ => BackwardsState::Empty,
                }
            }

            sum += digit;
        }

        sum.to_string()
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "281"
        }

        #[cfg(not(debug_assertions))]
        {
            "54885"
        }
    }

    fn day(&self) -> u8 {
        1
    }

    fn part(&self) -> u8 {
        2
    }
}
