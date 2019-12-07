use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    println!("{}", count_orbits(&ingest(file), "COM", 0));
}

fn ingest(file: BufReader<File>) -> HashMap<String, Vec<String>> {
    let mut system = HashMap::new();

    for line in file.lines().map(|l| l.unwrap()) {
        let orbit: Vec<&str> = line.split(")").collect();
        let parent = orbit.get(0).unwrap().to_string();

        system
            .entry(parent)
            .or_insert(Vec::new())
            .push(orbit.get(1).unwrap().to_string());
    }

    system
}

fn count_orbits(system: &HashMap<String, Vec<String>>, node: &str, depth: usize) -> usize {
    system
        .get(node)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|o| count_orbits(system, o, depth + 1))
        .sum::<usize>()
        + depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ingest() {
        let file = BufReader::new(File::open("test.txt").unwrap());

        let system = ingest(file);
        dbg!(system);
    }

    #[test]
    fn test_count() {
        let file = BufReader::new(File::open("test.txt").unwrap());

        let system = ingest(file);
        println!("{}", count_orbits(&system, "COM", 0));
    }
}
