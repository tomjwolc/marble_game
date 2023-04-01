use super::*;

pub fn pause_physics(
    mut rapier_config: ResMut<RapierConfiguration>
) {
    if let TimestepMode::Variable { time_scale, .. } =  &mut rapier_config.timestep_mode {
        *time_scale = 0.0;
    }
}

pub fn unpause_physics(
    mut rapier_config: ResMut<RapierConfiguration>
) {
    if let TimestepMode::Variable { time_scale, .. } =  &mut rapier_config.timestep_mode {
        *time_scale = 1.0;
    }
}