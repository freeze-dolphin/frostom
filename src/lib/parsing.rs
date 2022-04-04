use crate::lib::distance::Distance;
use osuparse::{Beatmap, HitCircle, HitObject, HoldNote, TimingPoint};
use std::collections::HashMap;

pub fn print_hit_objs(objs: Vec<HitObject>) {
    objs.iter().for_each(print_hit_obj);
}

pub fn print_hit_obj(obj: &HitObject) {
    println!("{}", hit_obj_to_string(obj));
}

pub fn hit_obj_to_string(obj: &HitObject) -> String {
    match obj {
        HitObject::HitCircle(obj) => format!(
            "HitCircle [x: {}, y: {}, new_combo: {}, color_skip: {}, time: {}]",
            obj.x, obj.y, obj.new_combo, obj.color_skip, obj.time
        ),
        HitObject::HoldNote(obj) => format!(
            "HoldNote [x: {}, y: {}, new_combo: {}, color_skip: {}, time: {}, end_time: {}]",
            obj.x, obj.y, obj.new_combo, obj.color_skip, obj.time, obj.end_time
        ),
        HitObject::Slider(_) => "Slider".to_string(),
        HitObject::Spinner(_) => "Spinner".to_string(),
    }
}

pub enum BeatCommand {
    Timing(TimingPoint),
    Hit(HitCircle),
    Hold(HoldNote),
}

pub type UnifiedBeatmap = HashMap<Distance, Vec<BeatCommand>>;

pub fn unify_beatmap(map: &Beatmap) -> UnifiedBeatmap {
    let mut u = UnifiedBeatmap::new();
    map.timing_points.iter().for_each(|f| {
        let ofs = f.offset as f64;
        let mut cont = u
            .get(&Distance::new(ofs))
            .unwrap_or_else(|| &(vec![] as Vec<BeatCommand>));
        cont.push(BeatCommand::Timing(TimingPoint { ..*f })); // three lines of codes cost me an hour to write :(
    });
    u
}
