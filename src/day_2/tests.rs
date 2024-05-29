use crate::{day_2, Part};

#[test]
fn day_2() {
    // Part One
    assert_eq!(day_2::day_2(Part::One,"src/day_2/data/example.txt"), 2);
    assert_eq!(day_2::day_2(Part::One,"src/day_2/data/input.txt"), 445);

    // Part Two
    assert_eq!(day_2::day_2(Part::Two, "src/day_2/data/example.txt"), 1);
    assert_eq!(day_2::day_2(Part::Two, "src/day_2/data/input.txt"), 491);
}