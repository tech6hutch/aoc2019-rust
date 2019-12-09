use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    io,
    num::ParseIntError,
};

type Int = i32;
type Intcode = Vec<Int>;

static PART1_INPUT: &str = include_str!("./part1/input");
const PART1_ANSWER: Int = 5074395;
const PART2_ANSWER: Int = 8346937;

pub fn day5() {
    println!("### Day 5 Part 1 ###");

    test_parse_program();
    test_run_program();

    let program = parse_program(PART1_INPUT).unwrap();
    run_program(program).expect("error running Intcode");
    println!("(The right output for input 1 is {})", PART1_ANSWER);
    println!("(The right output for input 5 is {})", PART2_ANSWER);
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
    UnknownParamMode,
    NegativePositionalParam,
    MutableParamInImmediateMode,
}

fn run_program(mut p: Intcode) -> Result<Intcode, RunProgramError> {
    use RunProgramError::*;

    let mut i = 0;
    loop {
        if i >= p.len() {
            return Err(NoHaltCode);
        }

        let (opcode, mut param_modes): (Int, Vec<ParamModes>) = {
            let n = next_int(&p, &mut i)?;
            let opcode = n % 100;
            let param_modes = (n / 100).to_digits()
                .into_iter()
                .map(ParamModes::from_int)
                .rev() // popping is more efficient than removing from the beginning
                .collect::<Option<_>>()
                .ok_or(RunProgramError::UnknownParamMode)?;
            (opcode, param_modes)
        };

        match opcode {
            // Add
            1 => {
                let mut params = take_params(&p, &mut i, 3)?;
                let a1 = resolve_param(&p, &mut params, &mut param_modes)?;
                let a2 = resolve_param(&p, &mut params, &mut param_modes)?;
                let out = resolve_param_mut(&mut p, &mut params, &mut param_modes)?;

                *out = a1 + a2;
            },

            // Multiply
            2 => {
                let mut params = take_params(&p, &mut i, 3)?;
                let f1 = resolve_param(&p, &mut params, &mut param_modes)?;
                let f2 = resolve_param(&p, &mut params, &mut param_modes)?;
                let out = resolve_param_mut(&mut p, &mut params, &mut param_modes)?;

                *out = f1 * f2;
            },

            // Input
            3 => {
                let mut params = take_params(&p, &mut i, 1)?;
                let out = resolve_param_mut(&mut p, &mut params, &mut param_modes)?;
                let input = get_input();
                *out = input;
            },

            // Output
            4 => {
                let mut params = take_params(&p, &mut i, 1)?;
                let out = resolve_param(&mut p, &mut params, &mut param_modes)?;
                output(out);
            },

            // Jump-if-true
            5 => {
                let mut params = take_params(&p, &mut i, 2)?;
                let condition = resolve_param(&p, &mut params, &mut param_modes)?;
                let addr = resolve_param(&p, &mut params, &mut param_modes)?;
                if condition != 0 {
                    i = addr.try_into().unwrap();
                }
            },

            // Jump-if-false
            6 => {
                let mut params = take_params(&p, &mut i, 2)?;
                let condition = resolve_param(&p, &mut params, &mut param_modes)?;
                let addr = resolve_param(&p, &mut params, &mut param_modes)?;
                if condition == 0 {
                    i = addr.try_into().unwrap();
                }
            },

            // Less than
            7 => {
                let mut params = take_params(&p, &mut i, 3)?;
                let first = resolve_param(&p, &mut params, &mut param_modes)?;
                let second = resolve_param(&p, &mut params, &mut param_modes)?;
                let out = resolve_param_mut(&mut p, &mut params, &mut param_modes)?;
                *out = if first < second { 1 } else { 0 };
            },

            // Equals
            8 => {
                let mut params = take_params(&p, &mut i, 3)?;
                let first = resolve_param(&p, &mut params, &mut param_modes)?;
                let second = resolve_param(&p, &mut params, &mut param_modes)?;
                let out = resolve_param_mut(&mut p, &mut params, &mut param_modes)?;
                *out = if first == second { 1 } else { 0 };
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

fn next_int(fake_iter: &[Int], i: &mut usize) -> Result<Int, RunProgramError> {
    let item = fake_iter.get(*i);
    *i += 1;
    match item {
        Some(&n) => Ok(n),
        None => Err(RunProgramError::NoHaltCode),
    }
}

fn take_params(fake_iter: &[Int], i: &mut usize, n: usize) -> Result<Vec<Int>, RunProgramError> {
    let items = fake_iter.get(*i..*i+n);
    *i += n;
    match items {
        Some(s) => Ok(s.iter().copied().rev().collect()),
        None => Err(RunProgramError::NoHaltCode),
    }
}

#[repr(i32)]
#[derive(FromPrimitive)]
enum ParamModes {
    /// The parameter is a position in the Intcode
    PositionMode = 0,
    /// The parameter is a literal value
    ImmediateMode = 1,
}

impl ParamModes {
    #[inline]
    fn from_int(n: Int) -> Option<Self> {
        num::FromPrimitive::from_i32(n)
    }
}

impl Default for ParamModes {
    fn default() -> Self {
        Self::PositionMode
    }
}

fn resolve_param(p: &Intcode, params: &mut Vec<Int>, modes: &mut Vec<ParamModes>) -> Result<Int, RunProgramError> {
    use RunProgramError::*;
    use ParamModes::*;

    let param = params.pop().expect("resolve_param() called too many times");
    match modes.pop().unwrap_or_default() {
        PositionMode => match usize::try_from(param) {
            Ok(addr)            => Ok(p[addr]),
            Err(_) if param < 0 => Err(NegativePositionalParam),
            Err(_)              => panic!("usize too small on this machine?"),
        },

        ImmediateMode => Ok(param),
    }
}

fn resolve_param_mut<'a>(p: &'a mut Intcode, params: &mut Vec<Int>, modes: &mut Vec<ParamModes>) -> Result<&'a mut Int, RunProgramError> {
    use RunProgramError::*;
    use ParamModes::*;

    let param = params.pop().expect("resolve_param_mut() called too many times");
    match modes.pop().unwrap_or_default() {
        PositionMode => match usize::try_from(param) {
            Ok(addr)            => Ok(&mut p[addr]),
            Err(_) if param < 0 => Err(NegativePositionalParam),
            Err(_)              => panic!("usize too small on this machine?"),
        },

        ImmediateMode => Err(MutableParamInImmediateMode),
    }
}

fn get_input() -> Int {
    use io::Write;

    print!("input> ");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.truncate_nl();

    s.parse().unwrap_or_else(|_| {
        println!("invalid integer");
        get_input()
    })
}

fn output(n: Int) {
    println!("{}", n);
}

trait ToDigits: Sized {
    fn to_digits(self) -> Vec<Self>;
    fn to_digits_in(self, v: Vec<Self>) -> Vec<Self>;
}

impl ToDigits for Int {
    #[inline]
    fn to_digits(self) -> Vec<Int> {
        self.to_digits_in(Vec::new())
    }
    #[inline]
    fn to_digits_in(self, mut v: Vec<Int>) -> Vec<Int> {
        match self {
            0 => v,
            _ => {
                v.push(self % 10);
                (self / 10).to_digits_in(v)
            }
        }
    }
}

trait TruncateNewLine {
    /// Remove a trailing newline, in-place.
    fn truncate_nl(&mut self);
}

impl TruncateNewLine for String {
    fn truncate_nl(&mut self) {
        if self.ends_with('\n') {
            self.pop();
            if self.ends_with('\r') {
                self.pop();
            }
        }
    }
}
