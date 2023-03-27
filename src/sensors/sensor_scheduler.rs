use super::*;

#[derive(Resource)]
pub struct SensorScheduler(pub Vec<SensorSchedule>);

pub struct SensorSchedule {
    pub is_active: bool,
    pub was_previously_active: bool,
    pub on_enter: Schedule,
    pub on_exit: Schedule
}