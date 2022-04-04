use std::fs;

use clap::Parser;

use crate::lib::cli::LaunchArgs;
use crate::lib::modes::{ModeBehavior, NormalMode, RawMode, TestMode};

pub mod lib;

fn main() {
    let args: LaunchArgs = LaunchArgs::parse();
    let map = args.input;

    let contents = fs::read_to_string(&map).expect("unable to read the map file");

    let map = osuparse::parse_beatmap(contents.as_str()).expect("unable to parse the map");

    if args.test {
        TestMode::run(map);
    } else if args.raw {
        RawMode::run(map);
    } else {
        NormalMode::run(map);
    }
}
