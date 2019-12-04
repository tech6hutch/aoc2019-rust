use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

static PART1_INPUT: &str = include_str!("./part1/input");
static PART1_ANSWER: i32 = 1084;

pub fn part1() {
    println!("### Day 3 Part 1 ###");

    test_path_to_points();
    test_closest_intersection_in_paths();

    let lines: Vec<&str> = PART1_INPUT.trim().lines().collect();

    let wire1 = path_to_points(lines[0]);
    let wire2 = path_to_points(lines[1]);
    let wire1_set = map_to_set(&wire1);
    let wire2_set = map_to_set(&wire2);
    let intersections: HashSet<(i32, i32)> = wire1_set.intersection(&wire2_set).cloned().collect();
    let closest_dist = intersections
        .iter()
        .min_by_key(|p| manhattan(**p))
        .cloned()
        .map(manhattan)
        .expect("no intersecting points???");
    assert_eq!(closest_dist, PART1_ANSWER);

    println!("The Manhattan distance from the central port to the closest intersection: {}", closest_dist);

    println!("### Day 3 Part 2 ###");

    let temp: Vec<u32> = intersections
        .iter()
        .map(|p| wire1[p] + wire2[p])
        .collect();
    println!("{:?}", temp);
    let closest_dist_fewest_steps = intersections
        .iter()
        .map(|p| wire1[p] + wire2[p])
        .min()
        .expect("huh");

    println!("The fewest combined steps the wires must take to reach an intersection: {}", closest_dist_fewest_steps);
}

fn map_to_set<K, V>(map: &HashMap<K, V>) -> HashSet<K>
    where K: Clone + Eq + std::hash::Hash
{
    map.keys().cloned().collect()
}

fn closest_intersection_in_paths(path1: &str, path2: &str) -> Option<(i32, i32)> {
    let wire1: HashSet<(i32, i32)> = path_to_points(path1).keys().cloned().collect();
    let wire2: HashSet<(i32, i32)> = path_to_points(path2).keys().cloned().collect();
    let intersections = wire1.intersection(&wire2);
    intersections.min_by_key(|p| manhattan(**p)).cloned()
}
fn test_closest_intersection_in_paths() {
    let tests: HashMap<(&str, &str), i32> = [
        (("R8,U5,L5,D3", "U7,R6,D4,L4"),
            6),
        (("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"),
            159),
        (("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
            135),
    ].iter().cloned().collect();

    for ((path1, path2), output) in tests {
        assert_eq!(closest_intersection_in_paths(path1, path2).map(manhattan), Some(output));
    }
}

fn path_to_points(path: &str) -> HashMap<(i32, i32), u32> {
    let mut list = HashMap::new();

    let (mut x, mut y, mut cnt) = (0, 0, 0);
    for step in path.split(',') {
        let (dir, dist): (char, i32) = {
            let (dir_str, dist_str) = step.split_at(1);
            (dir_str.chars().next().unwrap(),
                dist_str.parse().expect("couldn't parse distance number"))
        };

        cnt += 1;
        match dir {
            'L' => for _ in 0..dist {
                x -= 1;
                list.entry((x, y)).or_insert(cnt);
            },

            'R' => for _ in 0..dist {
                x += 1;
                list.entry((x, y)).or_insert(cnt);
            },

            'D' => for _ in 0..dist {
                y -= 1;
                list.entry((x, y)).or_insert(cnt);
            },

            'U' => for _ in 0..dist {
                y += 1;
                list.entry((x, y)).or_insert(cnt);
            },

            _ => panic!("unknown direction '{}'", dir)
        }
    }

    list
}
fn test_path_to_points() {
    let tests: HashMap<&str, HashSet<(i32, i32)>> = [
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
        assert_eq!(path_to_points(input).keys().cloned().collect::<HashSet<_>>(), output);
    }
}

/// Calculate Manhattan distance between a point and the origin
fn manhattan((x, y): (i32, i32)) -> i32 {
    x.abs() + y.abs()
}
