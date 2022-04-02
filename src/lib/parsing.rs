use osuparse::HitObject;

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
