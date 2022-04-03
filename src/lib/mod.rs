pub mod cli;
pub mod distance;
pub mod modes;
pub mod parsing;

#[cfg(test)]
mod tests {

    use osuparse::TimingPoint;
    use std::fs;

    fn print_timing_points(pnts: Vec<TimingPoint>) {
        pnts.iter().for_each(print_timing_point);
    }

    fn print_timing_point(pnt: &TimingPoint) {
        println!("TimingPoint [offset: {}, ms_per_beat: {}, meter: {}, sample_set: {}, sample_index: {}, volume: {}, inherited: {}, kiai_mode: {}]",
        pnt.offset, pnt.ms_per_beat, pnt.meter, pnt.sample_set, pnt.sample_index, pnt.volume, pnt.inherited, pnt.kiai_mode);
    }

    #[test]
    fn timing_points_test() {
        let map = osuparse::parse_beatmap(
            fs::read_to_string(
                "maps/1055661 Laur - Vindication/Laur - Vindication (Umo-) [Normal].osu",
            )
            .unwrap()
            .as_str(),
        )
        .unwrap();
        print_timing_points(map.timing_points);
        println!(
            "ver: {}, diff: {}",
            map.version, map.difficulty.overall_difficulty
        );
    }
}
