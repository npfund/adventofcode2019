use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    part1();
}

fn part1() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut memory = ingest(input.trim_end());

    *memory.get_mut(&1).unwrap() = 12;
    *memory.get_mut(&2).unwrap() = 2;

    println!("{}", execute(memory).get(&0).unwrap());
}

fn ingest(input: &str) -> HashMap<usize, usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .collect()
}

fn execute(mut memory: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut counter: usize = 0;
    while let Some(&opcode) = memory.get(&counter) {
        if opcode == 99 {
            break;
        }

        let lhs = get_value(&memory, get_value(&memory, counter + 1));
        let rhs = get_value(&memory, get_value(&memory, counter + 2));
        let destination = get_value(&memory, counter + 3);

        match opcode {
            1 => {
                memory.insert(destination, lhs + rhs);
            }
            2 => {
                memory.insert(destination, lhs * rhs);
            }
            _ => break,
        }

        counter += 4;
    }

    memory
}

fn get_value(memory: &HashMap<usize, usize>, address: usize) -> usize {
    memory.get(&address).unwrap().clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        dbg!(execute(ingest(input)));
    }

    #[test]
    fn test_execute_2() {
        let input = "1,1,1,4,99,5,6,0,99";
        dbg!(execute(ingest(input)));
    }
}
