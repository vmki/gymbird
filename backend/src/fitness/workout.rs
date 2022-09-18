use chrono::prelude::*;

pub struct Workout {
    name: String,
    datetime: chrono::DateTime<Local>,
}

pub struct TrainedMuscle {
    muscle: Muscle,
    priority: TrainedMusclePriority,
}

pub enum Muscle {
    Chest,
    UpperBack,
    Triceps,
    Biceps,
    LowerBack,
    Hamstrings,
    Quadriceps,
    Calves,
    Forearms,
    Core,
}

pub enum TrainedMusclePriority {
    Primary,
    Secondary,
    Stabilization,
}
