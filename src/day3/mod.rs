use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

static PART1_INPUT: &str = include_str!("./part1/input");
const PART1_ANSWER: i32 = 1084;
const PART2_ANSWER: u32 = 9240;

type Point = (i32, i32);
type Wire = HashMap<Point, u32>;

pub fn day3() {
    println!("### Day 3 Part 1 ###");

    test_path_to_points();

    let lines: Vec<&str> = PART1_INPUT.trim().lines().collect();

    let wire1 = parse_wire(lines[0]);
    let wire2 = parse_wire(lines[1]);
    let both = wires_intersection(wire1, wire2);
    let closest_dist = both
        .keys()
        .min_by_key(|p| manhattan(**p))
        .map(|p| manhattan(*p))
        .expect("no intersecting points???");
    assert_eq!(closest_dist, PART1_ANSWER);

    println!("The Manhattan distance from the central port to the closest intersection: {}", closest_dist);

    println!("### Day 3 Part 2 ###");

    let fewest_steps = both
        .values()
        .min()
        .copied()
        .expect("huh");
    assert_eq!(fewest_steps, PART2_ANSWER);

    println!("The fewest combined steps the wires must take to reach an intersection: {}", fewest_steps);
}

fn wires_intersection(wire1: Wire, wire2: Wire) -> Wire {
    wire1
        .iter()
        .filter_map(|(point, c1)| {
            wire2
                .get(point)
                .map(|c2| (point.clone(), c1 + c2))
        })
        .collect()
}

fn parse_wire(path: &str) -> Wire {
    let mut list = HashMap::new();

    let (mut x, mut y, mut steps) = (0, 0, 0);
    for cmd in path.split(',') {
        let (dir, dist): (char, i32) = parse_cmd(cmd);

        for _ in 0..dist {
            match dir {
                'L' => x -= 1, 'R' => x += 1,
                'D' => y -= 1, 'U' => y += 1,
                _ => panic!("unknown direction '{}'", dir)
            }
            let point = (x, y);
            steps += 1;
            if !list.contains_key(&point) {
                list.insert(point, steps);
            }
        }
    }

    list
}
fn test_path_to_points() {
    let tests: HashMap<&str, HashSet<Point>> = [
        ("R8,U5,L5,D3", vec![
            (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0), (8,0),
            (8,1), (8,2), (8,3), (8,4), (8,5),
            (7,5), (6,5), (5,5), (4,5), (3,5),
            (3,4), (3,3), (3,2),
        ]),
    ].iter().cloned().map(
        |(input, output)| (input, HashSet::from_iter(output))
    ).collect();

    for (input, output) in tests {
        assert_eq!(parse_wire(input).keys().cloned().collect::<HashSet<_>>(), output);
    }
}

fn parse_cmd(cmd: &str) -> (char, i32) {
    let (dir_str, dist_str) = cmd.split_at(1);
    (
        dir_str.chars().next().unwrap(),
        dist_str.parse().expect("couldn't parse distance number"),
    )
}

/// Calculate Manhattan distance between a point and the origin
fn manhattan((x, y): Point) -> i32 {
    x.abs() + y.abs()
}
