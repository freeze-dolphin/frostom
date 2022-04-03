use std::fs::File;
use std::process;
use std::time::Duration;

use cursive::event::Key;
use cursive::views::Dialog;
use osuparse::Beatmap;
use rodio::source::SineWave;
use rodio::{Decoder, OutputStream, Sink, Source};

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
    fn run(_map: Beatmap) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(
            Decoder::new(File::open("maps/1055661 Laur - Vindication/audio.mp3").unwrap()).unwrap(),
        );

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
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
