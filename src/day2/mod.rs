use std::num::ParseIntError;
use crate::day2::RunProgramError::{NoHaltCode, UnknownOpCode};
use std::collections::HashMap;

static PART1_INPUT: &str = include_str!("./part1/input");

type Intcode = Vec<u32>;

pub fn part1() {
    println!("### Day 2 Part 1 ###");
    
    test_parse_program();
    test_run_program();
    
    let mut program = parse_program(PART1_INPUT).unwrap();
    program[1] = 12;
    program[2] = 2;
    let program_after_run = run_program(program).unwrap();
    
    println!("Value at position 0: {}", program_after_run[0]);
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
                [a1, a2, out] =>
                    p[out as usize] = p[a1 as usize] + p[a2 as usize],
                _ =>
                    return Err(NoHaltCode)
            },
            2 => match p[i+1..=i+3] {
                [f1, f2, out] =>
                    p[out as usize] = p[f1 as usize] * p[f2 as usize],
                _ =>
                    return Err(NoHaltCode)
            },
            99 =>
                return Ok(p),
            _ =>
                return Err(UnknownOpCode)
        }
        
        i += 4;
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
