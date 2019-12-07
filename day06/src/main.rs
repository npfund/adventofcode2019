use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    println!("{}", count_orbits(&ingest(file), "COM", 0));
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let system = ingest(file);

    let path_to_you = find_path(&system, "COM", "YOU").unwrap();
    let path_to_santa = find_path(&system, "COM", "SAN").unwrap();

    let transfers = find_distance(&path_to_you, &path_to_santa);

    println!("{}", transfers);
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

fn find_path(
    system: &HashMap<String, Vec<String>>,
    node: &str,
    target: &str,
) -> Option<Vec<String>> {
    let orbits = system.get(node)?;

    let mut path = Vec::new();
    path.push(node.to_string());
    for orbit in orbits {
        if orbit == target {
            return Some(path);
        } else {
            match find_path(system, orbit, target) {
                Some(p) => {
                    path.extend(p.into_iter());
                    return Some(path);
                }
                None => {}
            }
        }
    }

    None
}

fn find_distance(path1: &Vec<String>, path2: &Vec<String>) -> usize {
    let path1_nodes: HashSet<String> = path1.clone().into_iter().collect();
    let path2_nodes: HashSet<String> = path2.clone().into_iter().collect();

    path1_nodes.symmetric_difference(&path2_nodes).count()
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

    #[test]
    fn test_find_path() {
        let file = BufReader::new(File::open("test2.txt").unwrap());

        let system = ingest(file);
        println!("{:?}", find_path(&system, "COM", "YOU"));
        println!("{:?}", find_path(&system, "COM", "SAN"));
    }

    #[test]
    fn test_find_distance() {
        let file = BufReader::new(File::open("test2.txt").unwrap());

        let system = ingest(file);
        let path_to_you = find_path(&system, "COM", "YOU").unwrap();
        let path_to_santa = find_path(&system, "COM", "SAN").unwrap();

        assert_eq!(4, find_distance(&path_to_you, &path_to_santa));
    }
}
