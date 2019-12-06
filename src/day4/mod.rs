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
            digits.windows(2).any(two_adjacent),
            digits.windows(2).all(never_decrease),
        )
    }).count().try_into().unwrap();
    assert_eq!(count, PART1_ANSWER);

    println!("How many different passwords: {}", count);
}

fn two_adjacent<T: PartialEq>(s: &[T]) -> bool {
    s[0] == s[1]
}

fn never_decrease<T: PartialOrd>(s: &[T]) -> bool {
    s[0] <= s[1]
}

pub fn part2() {
    println!("### Day 4 Part 2 ###");

    const MIN: u32 = 145852;
    const MAX: u32 = 616942;

    let count: u32 = (MIN..=MAX).filter(|n| {
        let digits = n
            .to_string()
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(digits.len(), 6);
        and!(
            only_two_adjacent_at_end(&digits[..=1], digits[2]) ||
                digits.windows(4).any(only_two_adjacent) ||
                only_two_adjacent_at_end(&digits[4..], digits[3]),
            digits.windows(2).all(never_decrease),
        )
    }).count().try_into().unwrap();

    println!("How many different passwords: {}", count);
}

fn only_two_adjacent<T: PartialEq>(s: &[T]) -> bool {
    assert_eq!(s.len(), 4);
    s[1] == s[2] &&
        s[0] != s[1] && s[2] != s[3]
}

fn only_two_adjacent_at_end<T: PartialEq>(two: &[T], next: T) -> bool {
    assert_eq!(two.len(), 2);
    two[0] == two[1] &&
        two[1] != next
}
