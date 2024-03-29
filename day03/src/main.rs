use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Segment {
    direction: (isize, isize),
    length: usize,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let closest = file
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(|s| {
                    let (dir, len) = s.split_at(1);

                    Segment {
                        direction: match dir {
                            "U" => (0, 1),
                            "D" => (0, -1),
                            "L" => (-1, 0),
                            "R" => (1, 0),
                            _ => panic!("Unknown direction {}", dir),
                        },
                        length: len.parse().unwrap(),
                    }
                })
                .collect::<Vec<Segment>>()
        })
        .map(|v: Vec<Segment>| {
            let mut current: (isize, isize) = (0, 0);
            v.iter().fold(HashSet::new(), |mut segment_grid, segment| {
                for _i in 0..segment.length {
                    current.0 += segment.direction.0;
                    current.1 += segment.direction.1;

                    segment_grid.insert(current);
                }
                segment_grid
            })
        })
        .fold(HashMap::new(), |mut grid, segment_grid| {
            for &coord in segment_grid.iter() {
                *grid.entry(coord).or_insert(0) += 1;
            }

            grid
        })
        .iter()
        .filter(|(&_coord, &count)| count > 1)
        .fold(99999999, |closest_distance, (&current, &_count)| {
            let current_distance = current.0.abs() + current.1.abs();
            if current_distance < closest_distance {
                current_distance
            } else {
                closest_distance
            }
        });

    println!("{}", closest);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let best = file
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(|s| {
                    let (dir, len) = s.split_at(1);

                    Segment {
                        direction: match dir {
                            "U" => (0, 1),
                            "D" => (0, -1),
                            "L" => (-1, 0),
                            "R" => (1, 0),
                            _ => panic!("Unknown direction {}", dir),
                        },
                        length: len.parse().unwrap(),
                    }
                })
                .collect::<Vec<Segment>>()
        })
        .map(|v: Vec<Segment>| {
            let mut current: (isize, isize) = (0, 0);
            let mut length = 1;
            v.iter().fold(HashMap::new(), |mut segment_grid, segment| {
                for _i in 0..segment.length {
                    current.0 += segment.direction.0;
                    current.1 += segment.direction.1;

                    segment_grid.entry(current).or_insert(length);
                    length += 1;
                }
                segment_grid
            })
        })
        .fold(HashMap::new(), |mut grid, segment_grid| {
            for (&coord, &length) in segment_grid.iter() {
                let entry = grid.entry(coord).or_insert((0, 0));
                *entry = (entry.0 + 1, entry.1 + length);
            }

            grid
        })
        .iter()
        .filter(|(&_coord, &(count, _length))| count > 1)
        .fold(99999999, |fewest_steps, (&_current, &(_count, length))| {
            if length < fewest_steps {
                length
            } else {
                fewest_steps
            }
        });

    println!("{}", best);
}
