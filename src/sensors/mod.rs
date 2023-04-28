pub use super::*;

mod sensor_events;
mod activatable;
mod activator;

pub use {
    sensor_events::*,
    activatable::*,
    activator::*
};

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(check_sensor_events.run_if(AppState::in_game))
            .add_systems((
                respawn_events,
                warp_events,
                activator_events,
                gravity_sensor_events,
                can_jump_sensor_events
            ).after(check_sensor_events).distributive_run_if(AppState::in_game))
            .add_system(update_activatables.after(activator_events))
            .add_systems((
                warp_activatable,
                button_activatable,

                // simple_activator,
                warp_activator
            ).after(update_activatables).distributive_run_if(AppState::in_game))
        ;
    }
}