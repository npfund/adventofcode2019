use crate::vm::Vm;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process;

mod vm;

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

    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    part1(&input);
}

fn part1(input: &str) {

    let vm = Vm::from(&input);
}
