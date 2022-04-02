use crate::print_hit_objs;
use osuparse::Beatmap;
use std::ops::Deref;
use std::process;

pub enum Mode {
    Raw(Beatmap),
    Normal(Beatmap),
}

impl Mode {
    pub fn run(&self) {
        match self {
            Mode::Raw(map) => {
                let cmap: Beatmap = *map;
                print_hit_objs(map.hit_objects);
                println!(
                    "ver: {}, diff: {}",
                    map.version, map.difficulty.overall_difficulty
                );

                process::exit(0);
            }
            Mode::Normal(map) => {}
        }
    }
}
