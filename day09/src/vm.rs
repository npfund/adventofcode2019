use std::collections::{HashMap, VecDeque};

pub struct Vm {
    memory: HashMap<usize, i128>,
    pointer: usize,
    input: VecDeque<i128>,
    relative_base: i128,
}

impl Vm {
    pub fn new(memory: HashMap<usize, i128>) -> Self {
        Vm {
            memory,
            pointer: 0,
            input: VecDeque::new(),
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, input: i128) {
        self.input.push_back(input);
    }

    pub fn execute(&mut self) -> Option<i128> {
        while let Some(&op) = self.memory.get(&self.pointer) {
            let opcode = Opcode::from(op);

            let length;
            match opcode.operation {
                1 => {
                    let lhs = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let rhs = self.get_entry(self.pointer + 2, opcode.get_mode(1));
                    let destination = self.get_entry_mut(self.pointer + 3, opcode.get_mode(2));

                    *destination = lhs + rhs;
                    length = 4;
                }
                2 => {
                    let lhs = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let rhs = self.get_entry(self.pointer + 2, opcode.get_mode(1));
                    let destination = self.get_entry_mut(self.pointer + 3, opcode.get_mode(2));

                    *destination = lhs * rhs;
                    length = 4;
                }
                3 => {
                    let arg = self.input.pop_front().expect("missing input");
                    let destination = self.get_entry_mut(self.pointer + 1, opcode.get_mode(0));

                    *destination = arg;
                    length = 2;
                }
                4 => {
                    let value = self.get_entry(self.pointer + 1, opcode.get_mode(0));

                    self.pointer += 2;
                    return Some(value);
                }
                5 => {
                    let value = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let destination = self.get_entry(self.pointer + 2, opcode.get_mode(1));

                    if value != 0 {
                        length = 0;
                        self.pointer = destination as usize;
                    } else {
                        length = 3;
                    }
                }
                6 => {
                    let value = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let destination = self.get_entry(self.pointer + 2, opcode.get_mode(1));

                    if value == 0 {
                        length = 0;
                        self.pointer = destination as usize;
                    } else {
                        length = 3;
                    }
                }
                7 => {
                    let lhs = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let rhs = self.get_entry(self.pointer + 2, opcode.get_mode(1));
                    let destination = self.get_entry_mut(self.pointer + 3, opcode.get_mode(2));

                    *destination = if lhs < rhs { 1 } else { 0 };
                    length = 4;
                }
                8 => {
                    let lhs = self.get_entry(self.pointer + 1, opcode.get_mode(0));
                    let rhs = self.get_entry(self.pointer + 2, opcode.get_mode(1));
                    let destination = self.get_entry_mut(self.pointer + 3, opcode.get_mode(2));

                    *destination = if lhs == rhs { 1 } else { 0 };
                    length = 4;
                }
                9 => {
                    let value = self.get_entry(self.pointer + 1, opcode.get_mode(0));

                    self.relative_base += value;

                    length = 2;
                }
                _ => {
                    break;
                }
            }

            self.pointer += length;
        }

        None
    }

    fn get_entry_mut(&mut self, address: usize, mode: ParameterMode) -> &mut i128 {
        if mode == ParameterMode::Immediate {
            self.memory.entry(address).or_insert(0)
        } else {
            let source = self.memory.get(&address).unwrap_or(&0).clone();
            let base = if mode == ParameterMode::Relative {
                self.relative_base
            } else {
                0
            };

            self.get_entry_mut((source + base) as usize, ParameterMode::Immediate)
        }
    }

    fn get_entry(&self, address: usize, mode: ParameterMode) -> i128 {
        if mode == ParameterMode::Immediate {
            *self.memory.get(&address).unwrap_or(&0)
        } else {
            let source = self.memory.get(&address).unwrap_or(&0).clone();
            let base = if mode == ParameterMode::Relative {
                self.relative_base
            } else {
                0
            };

            self.get_entry((source + base) as usize, ParameterMode::Immediate)
        }
    }
}

impl<T: Into<String>> From<T> for Vm {
    fn from(raw: T) -> Self {
        Vm::new(
            raw.into()
                .split(',')
                .map(|x| match x.trim().parse() {
                    Ok(int) => int,
                    Err(e) => panic!("{}, {}", e, x),
                })
                .enumerate()
                .collect(),
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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

impl From<i128> for Opcode {
    fn from(opcode: i128) -> Self {
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
                    2 => ParameterMode::Relative,
                    _ => panic!(),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_big_multiply() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut vm = Vm::from(input);

        let output = vm.execute();
        assert_eq!(Some(1219070632396864), output);
    }

    #[test]
    fn test_big_output() {
        let input = "104,1125899906842624,99";
        let mut vm = Vm::from(input);

        let output = vm.execute();
        assert_eq!(Some(1125899906842624), output);
    }

    #[test]
    fn test_immediate_multiplication() {
        let input = "1002,4,3,4,33";
        let mut vm = Vm::from(input);
        vm.execute();

        assert_eq!(*vm.memory.get(&4).unwrap(), 99);
    }

    #[test]
    fn test_immediate_addition() {
        let input = "1101,100,-1,4,0";
        let mut vm = Vm::from(input);
        vm.execute();

        assert_eq!(*vm.memory.get(&4).unwrap(), 99);
    }

    #[test]
    fn test_input_output() {
        let input = "3,0,4,0,99";
        let mut vm = Vm::from(input);
        vm.add_input(1234);

        let output = vm.execute();

        assert_eq!(Some(1234), output);
    }

    #[test]
    fn test_simple_addition() {
        let input = "1,0,0,0,99";
        let mut vm = Vm::from(input);
        vm.add_input(1234);

        vm.execute();

        assert_eq!(*vm.memory.get(&0).unwrap(), 2);
    }

    #[test]
    fn test_simple_multiplication() {
        let input = "2,3,0,3,99";
        let mut vm = Vm::from(input);
        vm.add_input(1234);

        vm.execute();

        assert_eq!(*vm.memory.get(&3).unwrap(), 6);
    }

    #[test]
    fn test_relative_address() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

        let mut vm = Vm::from(input);
        vm.add_input(1234);

        let mut output = Vec::new();
        while let Some(out) = vm.execute() {
            output.push(out);
        }

        let output = output.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
        assert_eq!(output, input);
    }
}
