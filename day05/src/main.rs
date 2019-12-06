use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process;
use log::debug;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {}.{} [{}:{}] #{} {}",
                chrono::Utc::now().to_rfc3339(),
                "INTCODE",
                record.level(),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                process::id(),
                message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    part1();
}

fn part1() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    execute(ingest(&input), Some(1));
}

fn ingest(input: &str) -> HashMap<usize, i32> {
    input
        .split(',')
        .map(|x| match x.trim().parse() { Ok(int) => int, Err(e) => panic!("{}, {}", e, x) })
        .enumerate()
        .collect()
}

fn execute(mut memory: HashMap<usize, i32>, input: Option<i32>) -> HashMap<usize, i32> {
    let mut counter: usize = 0;
    while let Some(&op) = memory.get(&counter) {
        let opcode = Opcode::from(op);

        let length;
        match opcode.operation {
            1 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!("{} {} {}: set position {} to {} + {}", op, get_address(&memory, counter + 1), get_address(&memory, counter + 2), destination, lhs, rhs);
                memory.insert(destination, lhs + rhs);
                length = 4;
            }
            2 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!("{} {} {}: set position {} to {} * {}", op, get_address(&memory, counter + 1), get_address(&memory, counter + 2), destination, lhs, rhs);
                memory.insert(destination, lhs * rhs);
                length = 4;
            }
            3 => {
                let destination = get_address(&memory, counter + 1);

                debug!("{}: set position {} to {}", op, destination, input.unwrap());
                memory.insert(destination, input.unwrap());
                length = 2;
            }
            4 => {
                let value = get_value(&memory, counter + 1, opcode.get_mode(0));

                debug!("{}: output {}", op, value);
                println!("{}", value);
                length = 2;
            }
            99 | _ => break,
        }

        counter += length;
    }

    memory
}

fn get_value(memory: &HashMap<usize, i32>, address: usize, mode: ParameterMode) -> i32 {
    let immediate = match memory.get(&address) {
        Some(v) => v,
        None => { panic!("unknown address {}", address) }
    };

    match mode {
        ParameterMode::Position => get_value(memory, *immediate as usize, ParameterMode::Immediate),
        ParameterMode::Immediate => immediate.clone(),
    }
}

fn get_address(memory: &HashMap<usize, i32>, address: usize) -> usize {
    get_value(memory, address, ParameterMode::Immediate) as usize
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Clone, Debug)]
struct Opcode {
    operation: usize,
    parameter_modes: Vec<ParameterMode>
}

impl Opcode {
    fn get_mode(&self, parameter: usize) -> ParameterMode {
        *self.parameter_modes.get(parameter).unwrap_or(&ParameterMode::Position)
    }
}

impl From<i32> for Opcode {
    fn from(opcode: i32) -> Self {
        let chars = opcode.to_string();
        let length = chars.len();
        let (params, op) = chars.split_at(if length > 2 { length - 2 } else { 0 });

        Opcode {
            operation: op.parse().unwrap(),
            parameter_modes: params.chars().rev().map(|c| match c.to_digit(10).unwrap() {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!(),
            }).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "1002,4,3,4,33";
        let result = execute(ingest(input), None);
        assert_eq!(*result.get(&4).unwrap(), 99);
    }

    #[test]
    fn test2() {
        let input = "1101,100,-1,4,0";
        let result = execute(ingest(input), None);
        assert_eq!(*result.get(&4).unwrap(), 99);
    }

    #[test]
    fn test_input_output() {
        let input = "3,0,4,0,99";
        execute(ingest(input), Some(1234));
    }
}
