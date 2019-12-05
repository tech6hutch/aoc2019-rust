const PART1_ANSWER: u32 = 1767;

pub fn part1() {
    println!("### Day 4 Part 1 ###");

    const MIN: u32 = 145852;
    const MAX: u32 = 616942;
    let mut min_digits: Vec<u32> = MIN.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let max_digits: Vec<u32> = MAX.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    assert!(!min_digits.iter().any(|d| *d > 9));
    assert!(!max_digits.iter().any(|d| *d > 9));
    for i in 1..min_digits.len() {
        min_digits[i] = min_digits[i].max(min_digits[i-1]);
    }
    assert!(!min_digits.iter().any(|d| *d > 9));

    let mut n = 0;
    for i in MIN..=MAX {
        let digits: Vec<char> = i.to_string().chars().collect();
        let mut digits_dont_increase = true;
        let mut has_2_same_digits = false;
        let mut prev_d = digits[0];
        for d in digits.iter().skip(1).copied() {
            if prev_d == d {
                has_2_same_digits = true;
            }
            if prev_d > d {
                digits_dont_increase = false;
                break;
            }
            prev_d = d;
        }
        if digits_dont_increase && has_2_same_digits {
            n += 1;
        }
    }
    // for d6 in min_digits[0]..=max_digits[0] {
    //     let last_d6 = d6 == max_digits[0];
    //     for d5 in min_digits[1].max(d6)..=(if last_d6 { max_digits[1] } else { 9 }) {
    //         let last_d5 = last_d6 && d5 == max_digits[1];
    //         let found2same = d5 == d6;
    //         for d4 in min_digits[2].max(d5)..=(if last_d5 { max_digits[2] } else { 9 }) {
    //             let last_d4 = last_d5 && d4 == max_digits[2];
    //             let found2same = found2same || d4 == d5;
    //             for d3 in min_digits[3].max(d4)..=(if last_d4 { max_digits[3] } else { 9 }) {
    //                 let last_d3 = last_d4 && d3 == max_digits[3];
    //                 let found2same = found2same || d3 == d4;
    //                 for d2 in min_digits[4].max(d3)..=(if last_d3 { max_digits[4] } else { 9 }) {
    //                     let last_d2 = last_d3 && d2 == max_digits[4];
    //                     let found2same = found2same || d2 == d3;
    //                     let d1_range = min_digits[5].max(d2)..=(if last_d2 { max_digits[5] } else { 9 });
    //                     if found2same {
    //                         let (len, len_opt) = d1_range.size_hint();
    //                         assert_eq!(len, len_opt.expect("range is unknown length"), "range size_hint lied >:(");
    //                         n += len;
    //                     } else if d1_range.contains(&d2) {
    //                         n += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    assert_eq!(n, PART1_ANSWER);

    println!("How many different passwords: {}", n);
}
