pub use super::*;

mod sensor_events;

pub use {
    sensor_events::*
};

pub struct SensorEvent {
    pub sensor_channel: SensorChannel,
    pub sensor_entity: Entity,
    pub object_entity: Entity
}

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SensorEvent>()
            .add_systems((
                check_sensor_events,
                respawn_sensor
            ).chain().distributive_run_if(AppState::in_game))
        ;
    }
}