static PART1_INPUT: &str = include_str!("./part1/input");
const PART1_ANSWER: u32 = 3325156;
const PART2_ANSWER: u32 = 4984866;

pub fn part1() {
    println!("### Day 1 Part 1 ###");

    let fuel_needed: u32 = get_module_masses()
        .into_iter()
        .filter_map(fuel_for_mass)
        .sum();
    assert_eq!(fuel_needed, PART1_ANSWER);

    println!("The sum of the fuel requirements: {}", fuel_needed);
}

fn get_module_masses() -> Vec<u32> {
    PART1_INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn fuel_for_mass(mass: u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

pub fn part2() {
    println!("### Day 1 Part 2 ###");

    let fuel_needed: u32 = get_module_masses()
        .into_iter()
        .map(fuel_for_mass_and_fuel)
        .sum();
    assert_eq!(fuel_needed, PART2_ANSWER);

    println!("The sum of the fuel requirements: {}", fuel_needed);
}

fn fuel_for_mass_and_fuel(mass: u32) -> u32 {
    fuel_for_mass(mass).map(fuel_for_fuel).unwrap_or_default()
}

fn fuel_for_fuel(fuel: u32) -> u32 {
    fuel + fuel_for_mass_and_fuel(fuel)
}
