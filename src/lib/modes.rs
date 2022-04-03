use crate::lib::distance::Distance;
use crossbeam::channel::tick;
use cursive::event::Key;
use cursive::views::Dialog;
use osuparse::Beatmap;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::time::Duration;
use std::{process, thread};

use crate::lib::parsing;
use crate::lib::parsing::UnifiedBeatmap;

pub trait ModeBehavior {
    fn run(map: Beatmap);
}

pub struct NormalMode;
pub struct RawMode;
pub struct TestMode;

impl ModeBehavior for RawMode {
    fn run(map: Beatmap) {
        parsing::print_hit_objs(map.hit_objects);
        println!(
            "ver: {}, diff: {}",
            map.version, map.difficulty.overall_difficulty
        );
        process::exit(0);
    }
}

impl ModeBehavior for TestMode {
    fn run(map: Beatmap) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let unified = parsing::unify_beatmap(&map);

        thread::spawn(|| {
            let ticker = tick(Duration::from_millis(1));
            let mut time_line = Distance::new(0.0);
        });

        sink.append(
            Decoder::new(File::open("maps/1055661 Laur - Vindication/audio.mp3").unwrap()).unwrap(),
        );

        sink.sleep_until_end();
    }
}

impl ModeBehavior for NormalMode {
    fn run(_map: Beatmap) {
        let mut siv = cursive::default();

        siv.add_global_callback(Key::Esc, |s| s.quit());
        siv.add_layer(Dialog::text("Select a "));

        siv.run();
    }
}
