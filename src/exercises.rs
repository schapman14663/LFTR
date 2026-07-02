struct Exercise {
    name: string,
    setup: string,
    equipment: string,
    set: struct
}

struct Set {
    weight: f32,
    high_target_reps: f32,
    low_target_reps: f32,
    actual_reps: f32,
    progression_weight: f32,
    notes: string,
    rest: f32
}
