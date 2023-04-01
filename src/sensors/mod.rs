pub use super::*;

mod sensor_events;
mod activation;

pub use {
    sensor_events::*,
    activation::*
};

pub struct RespawnEvent;

#[derive(Debug)]
pub struct WarpEvent {
    warp_to: WarpTo,
    object_entity: Entity
}

pub struct ActivatorEvent(pub usize);

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RespawnEvent>()
            .add_event::<WarpEvent>()
            .add_event::<ActivatorEvent>()
            .add_system(check_sensor_events.run_if(AppState::in_game))
            .add_systems((
                respawn_events,
                warp_events,
                activator_events
            ).after(check_sensor_events).distributive_run_if(AppState::in_game))
            .add_system(activate_activatables.after(activator_events).run_if(AppState::in_game))
            .add_systems((
                warp_activation,
                engage_activator
            ).after(activate_activatables).distributive_run_if(AppState::in_game))
        ;
    }
}