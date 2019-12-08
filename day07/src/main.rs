use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process;

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

    let memory = ingest(&input);

    let mut max = 0;
    let mut sequence = (0, 0, 0, 0, 0);

    for phase1 in 0..5 {
        let output = execute(memory.clone(), vec![phase1, 0]);
        for phase2 in 0..5 {
            if phase2 == phase1 {
                continue;
            }
            let output = execute(
                memory.clone(),
                vec![phase2, *output.1.last().expect("missing amp 1 output")],
            );
            for phase3 in 0..5 {
                if phase3 == phase2 || phase3 == phase1 {
                    continue;
                }
                let output = execute(
                    memory.clone(),
                    vec![phase3, *output.1.last().expect("missing amp 2 output")],
                );
                for phase4 in 0..5 {
                    if phase4 == phase3 || phase4 == phase2 || phase4 == phase1 {
                        continue;
                    }
                    let output = execute(
                        memory.clone(),
                        vec![phase4, *output.1.last().expect("missing amp 3 output")],
                    );
                    for phase5 in 0..5 {
                        if phase5 == phase4
                            || phase5 == phase3
                            || phase5 == phase2
                            || phase5 == phase1
                        {
                            continue;
                        }
                        let output = execute(
                            memory.clone(),
                            vec![phase5, *output.1.last().expect("missing amp 4 output")],
                        );
                        let amp5_output = *output.1.last().unwrap();
                        if amp5_output > max {
                            max = *output.1.last().expect("missing amp 5 output");
                            sequence = (phase1, phase2, phase3, phase4, phase5);
                        }
                    }
                }
            }
        }
    }

    println!("{} {:?}", max, sequence);
}

fn ingest(input: &str) -> HashMap<usize, i32> {
    input
        .split(',')
        .map(|x| match x.trim().parse() {
            Ok(int) => int,
            Err(e) => panic!("{}, {}", e, x),
        })
        .enumerate()
        .collect()
}

fn execute(
    mut memory: HashMap<usize, i32>,
    mut input: Vec<i32>,
) -> (HashMap<usize, i32>, Vec<i32>) {
    debug!("input: {:?}", input);
    input.reverse();
    let mut output = Vec::new();
    let mut counter: usize = 0;
    while let Some(&op) = memory.get(&counter) {
        let opcode = Opcode::from(op);

        let length;
        match opcode.operation {
            1 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!(
                    "{} {} {}: set position {} to {} + {}",
                    op,
                    get_address(&memory, counter + 1),
                    get_address(&memory, counter + 2),
                    destination,
                    lhs,
                    rhs
                );
                memory.insert(destination, lhs + rhs);
                length = 4;
            }
            2 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!(
                    "{} {} {}: set position {} to {} * {}",
                    op,
                    get_address(&memory, counter + 1),
                    get_address(&memory, counter + 2),
                    destination,
                    lhs,
                    rhs
                );
                memory.insert(destination, lhs * rhs);
                length = 4;
            }
            3 => {
                let destination = get_address(&memory, counter + 1);

                let arg = input.pop().unwrap();
                debug!("{}: set position {} to {}", op, destination, arg);
                memory.insert(destination, arg);
                length = 2;
            }
            4 => {
                let value = get_value(&memory, counter + 1, opcode.get_mode(0));

                debug!("{}: output {}", op, value);
                output.push(value);
                length = 2;
            }
            5 => {
                let value = get_value(&memory, counter + 1, opcode.get_mode(0));
                let destination = get_value(&memory, counter + 2, opcode.get_mode(1));

                debug!("{}: if {} != 0 goto {}", op, value, destination);
                if value != 0 {
                    length = 0;
                    counter = destination as usize;
                } else {
                    length = 3;
                }
            }
            6 => {
                let value = get_value(&memory, counter + 1, opcode.get_mode(0));
                let destination = get_value(&memory, counter + 2, opcode.get_mode(1));

                debug!("{}: if {} == 0 goto {}", op, value, destination);
                if value == 0 {
                    length = 0;
                    counter = destination as usize;
                } else {
                    length = 3;
                }
            }
            7 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!(
                    "{} {} {}: if {} < {} set position {} to 1 else 0",
                    op,
                    get_address(&memory, counter + 1),
                    get_address(&memory, counter + 2),
                    lhs,
                    rhs,
                    destination
                );
                memory.insert(destination, if lhs < rhs { 1 } else { 0 });
                length = 4;
            }
            8 => {
                let lhs = get_value(&memory, counter + 1, opcode.get_mode(0));
                let rhs = get_value(&memory, counter + 2, opcode.get_mode(1));
                let destination = get_address(&memory, counter + 3);

                debug!(
                    "{} {} {}: if {} == {} set position {} to 1 else 0",
                    op,
                    get_address(&memory, counter + 1),
                    get_address(&memory, counter + 2),
                    lhs,
                    rhs,
                    destination
                );
                memory.insert(destination, if lhs == rhs { 1 } else { 0 });
                length = 4;
            }
            o => {
                debug!("{}: exiting", o);
                break;
            }
        }

        counter += length;
    }

    (memory, output)
}

fn get_value(memory: &HashMap<usize, i32>, address: usize, mode: ParameterMode) -> i32 {
    let immediate = match memory.get(&address) {
        Some(v) => v,
        None => panic!("unknown address {}", address),
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
    parameter_modes: Vec<ParameterMode>,
}

impl Opcode {
    fn get_mode(&self, parameter: usize) -> ParameterMode {
        *self
            .parameter_modes
            .get(parameter)
            .unwrap_or(&ParameterMode::Position)
    }
}

impl From<i32> for Opcode {
    fn from(opcode: i32) -> Self {
        let chars = opcode.to_string();
        let length = chars.len();
        let (params, op) = chars.split_at(if length > 2 { length - 2 } else { 0 });

        Opcode {
            operation: op.parse().unwrap(),
            parameter_modes: params
                .chars()
                .rev()
                .map(|c| match c.to_digit(10).unwrap() {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    _ => panic!(),
                })
                .collect(),
        }
    }
}
