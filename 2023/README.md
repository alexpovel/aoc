# Advent of Code 2023

Written in Rust.

Going for performance, but not at the expense of readability and idiomatic code.
Trying for a balance... Current numbers (AMD Ryzen 7 5800X3D, DDR4 @ 3600MHz):

```text
✅ Day 1 / Part 1: 54697  (took 54.719µs)
✅ Day 1 / Part 2: 54885  (took 90.89µs)
✅ Day 2 / Part 1: 3035  (took 39.949µs)
✅ Day 2 / Part 2: 66027  (took 34.99µs)
✅ Day 3 / Part 1: 512794  (took 122.529µs)
✅ Day 3 / Part 2: 67779080  (took 686.595µs)
✅ Day 4 / Part 1: 26426  (took 51.8µs)
✅ Day 4 / Part 2: 6227972  (took 56.969µs)
✅ Day 5 / Part 1: 289863851  (took 39.78µs)
✅ Day 5 / Part 2: 60568880  (took 56.58µs)
✅ Day 6 / Part 1: 3316275  (took 1.06µs)
✅ Day 6 / Part 2: 27102791  (took 3.95µs)
✅ Day 7 / Part 1: 250347426  (took 327.898µs)
✅ Day 7 / Part 2: 251224870  (took 298.928µs)
✅ Day 8 / Part 1: 12361  (took 180.449µs)
✅ Day 8 / Part 2: 18215611419223  (took 1.30852ms)
✅ Day 9 / Part 1: 1974913025  (took 66.36µs)
✅ Day 9 / Part 2: 884  (took 63.059µs)
✅ Day 10 / Part 1: 6951  (took 243.448µs)
✅ Day 11 / Part 1: 9370588  (took 93.75µs)
✅ Day 11 / Part 2: 746207878188  (took 98.689µs)

Total time:     3.920912ms
```

Produced with `cargo run --release`.

Notes:

- inputs are compiled into the binary as `&'static str` literals, so solving times do
  not include any I/O (beyond the OS `mmap`ing the binary itself; run at least twice to
  get cached results)
- solving times *do* include parsing the `&'static str` input into whatever format is
  needed (into `Vec`s, `struct`s, parsing numbers, ...)
- parts of a day are generally solved entirely independently; the parsing step is
  **repeated** each time (generally copy-pasted code).

  If parsing is identical between parts, this doubles efforts. This is not corrected
  for. However, cases where parsing *differs* (e.g., part 1 works using line-by-line
  parsing, part 2 requires parsing the entire input for sorting) *benefit* from this
  approach (but the former case is much more common).
