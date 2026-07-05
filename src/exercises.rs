struct Exercise {
    name: String,
    setup: String,
    equipment: String,
    sets: Struct //This should probably be a list/array of structs
}

enum TypeOfSet{
    Weight_Set,
    Timed_Set,
}

struct Timed_Set {
    count_in: u8, //The problem I've had with certain apps is that you hit go and it immediately
    //starts the timer even if you need time to get set up, this gets around that.
    set_time: u16,
    reps_performed: u16 //just in cse you're supposed to be counting reps.
}

struct Weight_Set { //I honestly don't know if I need to make this mutable yet, the intention is to
    //just create new sets rather than changing old ones. 
    weight: f32,
    high_target_reps: u16,
    low_target_reps: u16,
    actual_reps: u16, //We don't do half reps here.
    progression_weight: f32, //We do, however, use 1.25kg and 2.5kg plates
    notes: String,
    rest: u16
}

impl Weight_Set {
    fn increment_weight_set(&self) -> Weight_Set {
        let weight_set2 = Weight_Set {
            weight = weight + progression_weight,
            notes = " ",
            ..&self
        }
    }
}
