use std::convert::TryInto;

/// So that things don't look ugly
macro_rules! and {
    ($b1:expr, $b2:expr,) => {
        $b1 && $b2
    };
}

const PART1_ANSWER: u32 = 1767;

pub fn part1() {
    println!("### Day 4 Part 1 ###");

    const MIN: u32 = 145852;
    const MAX: u32 = 616942;

    let count: u32 = (MIN..=MAX).filter(|n| {
        let digits = n
            .to_string()
            .chars()
            .collect::<Vec<_>>();
        and!(
            digits.windows(2).any(|d| d[0] == d[1]),
            digits.windows(2).all(|d| d[0] <= d[1]),
        )
    }).count().try_into().unwrap();
    assert_eq!(count, PART1_ANSWER);

    println!("How many different passwords: {}", count);
}
