pub use super::*;

mod sensor_events;
mod activation;

pub use {
    sensor_events::*,
    activation::*
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
            .add_system(activate_activatables.after(activator_events).run_if(AppState::in_game))
            .add_systems((
                warp_activation,
                engage_activator
            ).after(activate_activatables).distributive_run_if(AppState::in_game))
        ;
    }
}