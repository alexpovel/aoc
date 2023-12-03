pub mod day03;
pub mod iter;

trait Challenge {
    fn input() -> &'static str
    where
        Self: Sized;

    fn solve(&self) -> String;

    fn solution(&self) -> &'static str;

    fn day(&self) -> u8;

    fn part(&self) -> u8;

    fn title(&self) -> String {
        format!("Day {} / Part {}", self.day(), self.part())
    }
}

fn main() {
    let challenges: Vec<Box<dyn Challenge>> = vec![
        Box::new(day03::part1::Part {}),
        Box::new(day03::part2::Part {}),
    ];

    let mut total = std::time::Duration::default();

    let mut fail = false;
    for challenge in challenges {
        use std::time::Instant;
        let now = Instant::now();
        let solution = challenge.solve();
        let elapsed = now.elapsed();
        total += elapsed;

        let mark = if solution == challenge.solution() {
            "✅"
        } else {
            fail = true;

            "❌"
        };

        println!(
            "{} {}: {}\t(took\t{:?})",
            mark,
            challenge.title(),
            solution,
            elapsed
        );
    }

    println!("\nTotal time:\t{:?}", total);

    if fail {
        std::process::exit(1);
    }
}
