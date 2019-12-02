use std::{collections::HashMap, num::ParseIntError};
use crate::day2::RunProgramError::{NoHaltCode, UnknownOpCode};

type Int = u32;
type Intcode = Vec<Int>;

static PART1_INPUT: &str = include_str!("./part1/input");
const PART1_ANSWER: Int = 4090689;
const PART2_ANSWER: Int = 7733;

pub fn part1() {
    println!("### Day 2 Part 1 ###");

    test_parse_program();
    test_run_program();

    let mut memory = parse_program(PART1_INPUT).unwrap();
    memory[1] = 12;
    memory[2] = 2;
    memory = run_program(memory).unwrap();
    let output = memory[0];
    assert_eq!(output, PART1_ANSWER);

    println!("Value at position 0: {}", output);
}

fn parse_program(p: &str) -> Result<Intcode, ParseIntError> {
    p
        .trim()
        .split(',')
        .map(|s| s.parse())
        .collect()
}
fn test_parse_program() {
    let tests: HashMap<&str, Intcode> = [
        ("1,0,0,3,99", vec![1,0,0,3,99]),
        ("1,9,10,3,2,3,11,0,99,30,40,50", vec![1,9,10,3,2,3,11,0,99,30,40,50]),
        ("1,1,1,4,99,5,6,0,99", vec![1,1,1,4,99,5,6,0,99]),
    ].iter().cloned().collect();
    for (input, output) in tests {
        assert_eq!(parse_program(input).unwrap(), output);
    }
}

#[derive(Debug)]
enum RunProgramError {
    NoHaltCode,
    UnknownOpCode,
}

fn run_program(mut p: Intcode) -> Result<Intcode, RunProgramError> {
    let mut i = 0;
    loop {
        if i >= p.len() {
            return Err(NoHaltCode);
        }

        let opcode = p[i];
        match opcode {
            1 => match p[i+1..=i+3] {
                [a1, a2, out] => {
                    p[out as usize] = p[a1 as usize] + p[a2 as usize];
                    i += 4;
                },
                _ =>
                    return Err(NoHaltCode)
            },

            2 => match p[i+1..=i+3] {
                [f1, f2, out] => {
                    p[out as usize] = p[f1 as usize] * p[f2 as usize];
                    i += 4;
                },
                _ =>
                    return Err(NoHaltCode)
            },

            99 =>
                return Ok(p),

            _ =>
                return Err(UnknownOpCode)
        }
    }
}
fn test_run_program() {
    let tests: HashMap<Intcode, Intcode> = [
        (vec![1,9,10,3,2,3,11,0,99,30,40,50], vec![3500,9,10,70,2,3,11,0,99,30,40,50]),
        (vec![1,0,0,0,99], vec![2,0,0,0,99]),
        (vec![2,3,0,3,99], vec![2,3,0,6,99]),
        (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
        (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
    ].iter().cloned().collect();
    for (input, output) in tests {
        assert_eq!(run_program(input).unwrap(), output);
    }
}

pub fn part2() {
    println!("### Day 2 Part 2 ###");

    const NEEDED_OUTPUT: Int = 19690720;
    if let Some((noun, verb)) = find_noun_verb(parse_program(PART1_INPUT).unwrap(), NEEDED_OUTPUT) {
        let answer = 100 * noun + verb;
        assert_eq!(answer, PART2_ANSWER);
        println!("100 * noun + verb: {}", answer);
    } else {
        eprintln!("Error: Could not find the correct noun and verb.");
    }
}

fn find_noun_verb(memory: Intcode, needed_output: Int) -> Option<(Int, Int)> {
    for n in 0..=100 {
        for v in 0..=100 {
            match run_with_noun_verb(memory.clone(), n, v) {
                Ok(output) if output == needed_output =>
                    return Some((n, v)),

                _ => {}
            }
        }
    }
    return None;
}

fn run_with_noun_verb(mut p: Intcode, noun: Int, verb: Int) -> Result<Int, RunProgramError> {
    p[1] = noun;
    p[2] = verb;
    run_program(p).map(|pp| pp[0])
}
