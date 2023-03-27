pub use super::*;

mod sensor_events;
mod sensor_scheduler;

pub use {
    sensor_events::*,
    sensor_scheduler::*
};

pub struct RespawnEvent;

#[derive(Debug)]
pub struct WarpEvent {
    warp_to: WarpTo,
    object_entity: Entity
}

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RespawnEvent>()
            .add_event::<WarpEvent>()
            .add_system(check_sensor_events.run_if(AppState::in_game))
            .add_systems((
                respawn_events,
                warp_events
            ).before(check_sensor_events).distributive_run_if(AppState::in_game))
        ;
    }
}