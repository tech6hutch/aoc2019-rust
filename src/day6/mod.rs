use std::{
    collections::HashMap,
    str,
};

static PART1_INPUT: &str = include_str!("./part1/input");
const PART1_ANSWER: usize = 292387;

pub fn part1() {
    println!("### Day 6 Part 1 ###");

    let orbits: HashMap<&str, Vec<&str>> = PART1_INPUT
        .trim()
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let (object, orbiter) = line.split(')').tuple2().expect("invalid orbit syntax");
            map.entry(object).or_default().push(orbiter);
            map
        });
    let orbit_count = count_indirect_orbits(&orbits);
    assert_eq!(orbit_count, PART1_ANSWER);

    println!("Orbit count: {}", orbit_count);
}

fn count_indirect_orbits(orbits: &HashMap<&str, Vec<&str>>) -> usize {
    _count_indirect_orbits(orbits, "COM", 1)
}
fn _count_indirect_orbits(orbits: &HashMap<&str, Vec<&str>>, object: &str, orbit_depth: usize) -> usize {
    orbits
        .get(object)
        .map(|v| orbit_depth * v.len() + v.iter().map(|&k| _count_indirect_orbits(&orbits, k, orbit_depth + 1)).sum::<usize>())
        .unwrap_or_default()
}

trait ToTuple<T> {
    fn tuple2(self) -> Option<(T, T)>;
}

impl<T: Copy> ToTuple<T> for &[T] {
    fn tuple2(self) -> Option<(T, T)> {
        match self {
            &[a, b] => Some((a, b)),
            _ => None
        }
    }
}

impl<T: Copy> ToTuple<T> for Vec<T> {
    fn tuple2(self) -> Option<(T, T)> {
        self[..].tuple2()
    }
}

impl<'a, P> ToTuple<&'a str> for str::Split<'a, P>
    where P: str::pattern::Pattern<'a>
{
    fn tuple2(mut self) -> Option<(&'a str, &'a str)> {
        match (self.next(), self.next(), self.next()) {
            (Some(a), Some(b), None) => Some((a, b)),
            _ => None
        }
    }
}
