pub mod day01;
pub mod day03;
pub mod day04;
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
        Box::new(day01::part1::Part {}),
        Box::new(day01::part2::Part {}),
        Box::new(day03::part1::Part {}),
        Box::new(day03::part2::Part {}),
        Box::new(day04::part1::Part {}),
        Box::new(day04::part2::Part {}),
    ];

    let mut total = std::time::Duration::default();

    let mut fail = false;
    for challenge in challenges {
        use std::time::Instant;
        let now = Instant::now();
        let solution = challenge.solve();
        let elapsed = now.elapsed();
        total += elapsed;

        let actual_solution = challenge.solution();
        let (mark, actual_solution_hint) = if solution == actual_solution {
            ("✅", "".to_string())
        } else {
            fail = true;

            ("❌", format!("(should be {})", actual_solution))
        };

        println!(
            "{} {}: {} {} (took {:?})",
            mark,
            challenge.title(),
            solution,
            actual_solution_hint,
            elapsed
        );
    }

    println!("\nTotal time:\t{:?}", total);

    if fail {
        std::process::exit(1);
    }
}
