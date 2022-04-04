use crate::lib::distance::Distance;
use crossbeam::channel::tick;
use cursive::event::Key;
use cursive::views::Dialog;
use osuparse::Beatmap;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::time::{Duration, Instant};
use std::{process, thread};

use crate::lib::parsing;

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
            let start = Instant::now();
            let ticker = tick(Duration::from_millis(100));

            for _ in 0..5 {
                let msg = ticker.recv().unwrap();
                println!("{:?} elapsed: {:?}", msg, start.elapsed());
            }
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
