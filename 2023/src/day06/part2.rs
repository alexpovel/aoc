use crate::Challenge;

/// --- Part Two ---
///
/// As the race is about to start, you realize the piece of paper with race times and
/// record distances you got earlier actually just has very bad kerning. There's really
/// only one race - ignore the spaces between the numbers on each line.
///
/// So, the example from before:
///
/// ```text
/// Time:      7  15   30
/// Distance:  9  40  200
/// ```
///
/// ...now instead means this:
///
/// ```text
/// Time:      71530
/// Distance:  940200
/// ````
///
/// Now, you have to figure out how many ways there are to win this single race. In this
/// example, the race lasts for 71530 milliseconds and the record distance you need to
/// beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516
/// milliseconds and beat the record, a total of 71503 ways!
///
/// How many ways can you beat the record in this one much longer race?
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
        6
    }

    fn part(&self) -> u8 {
        2
    }

    fn solution(&self) -> &'static str {
        #[cfg(debug_assertions)]
        {
            "71503"
        }

        #[cfg(not(debug_assertions))]
        {
            "27102791"
        }
    }

    fn solve(&self) -> String {
        let input = Self::input();

        let input = input.strip_prefix("Time:").unwrap();
        let (times, input) = input.split_once('\n').unwrap();
        let times: String = times.split_ascii_whitespace().collect();
        let time = times.parse::<f64>().unwrap();

        let input = input.strip_prefix("Distance:").unwrap();
        let (distances, _input) = input.split_once('\n').unwrap();
        let distances: String = distances.split_ascii_whitespace().collect();
        let record_distance = distances.parse::<f64>().unwrap();

        // Roots of quadratic equation
        let upper = ((time + (time.powi(2) - 4.0 * (record_distance + 1.0)).sqrt()) / 2.0).floor();
        let lower = time - upper;

        let n = (upper.floor() - lower.ceil() + 1.0).floor() as u64;

        n.to_string()
    }
}
