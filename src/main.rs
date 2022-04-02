use std::fs;

use clap::Parser;
use osuparse::parse_beatmap;

use lib::cli::LaunchArgs;

use crate::lib::modes::Mode;
use crate::lib::parsing::print_hit_objs;

pub mod lib;

fn main() {
    let args: LaunchArgs = LaunchArgs::parse();
    let map = args.input;

    let contents = fs::read_to_string(&map).expect("unable to read the map file");

    let parsed = parse_beatmap(contents.as_str()).expect("unable to parse the map");

    if args.raw {
        Mode::Raw(parsed).run();
    } else {
        Mode::Normal(parsed).run();
    }
}
