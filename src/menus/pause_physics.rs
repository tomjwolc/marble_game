use super::*;

pub fn pause_physics(
    mut unlocked_axis_query: Query<&mut LockedAxes, (With<InGameEntity>, Without<Sensor>)>
) {
    for mut locked_axes in unlocked_axis_query.iter_mut() {
        *locked_axes = LockedAxes::TRANSLATION_LOCKED;
    }
}

pub fn un_pause_physics(
    mut locked_axis_query: Query<&mut LockedAxes, (With<InGameEntity>, Without<Sensor>)>
) {
    for mut locked_axes in locked_axis_query.iter_mut() {
        *locked_axes = LockedAxes::empty();
    }
}