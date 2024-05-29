use crate::{day_1, Part};

#[test]
fn day_1() {
    // Part 1
    assert_eq!(day_1::day_1(Part::One, "src/day_1/data/example.txt"), 514579);
    assert_eq!(day_1::day_1(Part::One, "src/day_1/data/input.txt"), 980499);

    // Part 2
    assert_eq!(day_1::day_1(Part::Two, "src/day_1/data/example.txt"), 241861950);
    assert_eq!(day_1::day_1(Part::Two, "src/day_1/data/input.txt"), 200637446);
}