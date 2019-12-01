use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    println!(
        "{}",
        file.lines()
            .map(|x| x.unwrap().parse::<f64>().unwrap())
            .map(|x| (x / 3.0).trunc() - 2.0)
            .sum::<f64>()
    );
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    println!(
        "{}",
        file.lines()
            .map(|x| x.unwrap().parse::<f64>().unwrap())
            .map(|module| {
                let mut steps = Vec::new();
                let mut required = (module / 3.0).trunc() - 2.0;
                while required > 0.0 {
                    steps.push(required);
                    required = (required / 3.0).trunc() - 2.0;
                }

                steps.iter().sum::<f64>()
            })
            .sum::<f64>()
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fuel_fuel() {
        let module: f64 = 100756.0;
        let mut steps = Vec::new();
        let mut required = (module / 3.0).trunc() - 2.0;
        while required > 0.0 {
            steps.push(required);
            required = (required / 3.0).trunc() - 2.0;
        }

        println!("{} {}", module, steps.iter().sum::<f64>());
    }
}
