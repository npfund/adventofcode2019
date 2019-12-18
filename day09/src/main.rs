use crate::vm::Vm;
use std::fs::File;
use std::io::Read;

mod vm;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, _record| out.finish(format_args!("{}", message,)))
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    part1(&input);
}

fn part1(input: &str) {
    let mut vm = Vm::from(input);
    vm.add_input(1);

    while let Some(output) = vm.execute() {
        println!("{}", output);
    }
}
