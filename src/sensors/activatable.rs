use super::*;

pub fn warp_activatable(
    mut activatable_query: Query<(&Activatable, &mut SensorChannel), (With<Sensor>, With<WarpTo>, With<InGameEntity>)>
) {
    for (
        &Activatable { is_active, .. }, 
        mut sensor_channel
    ) in activatable_query.iter_mut() {
        *sensor_channel = if is_active { SensorChannel::Warp } else { SensorChannel::None };
    }
}

// Compresses the physical button when it is activated
pub fn button_activatable(
    mut activatable_query: Query<(&Activatable, &mut Transform, &ButtonActivatable), With<InGameEntity>>
) {
    for (
        &Activatable { is_active, .. }, 
        mut transform,
        &ButtonActivatable { initial_position }
    ) in activatable_query.iter_mut() {
        transform.translation = initial_position + transform.down() * if is_active { BUTTON_COMPRESS_DEPTH } else { 0.0 };
    }
}

pub fn update_activatables(
    activator_query: Query<&Activator, (Changed<Activator>, With<InGameEntity>)>,
    mut activatable_query: Query<&mut Activatable, With<InGameEntity>>,
) {
    let mut activation_table = HashSet::new();

    for Activator { id, is_active, .. } in activator_query.iter() {
        if *is_active { activation_table.insert(*id); }
    }

    for mut activatable in activatable_query.iter_mut() {
        activatable.is_active = activatable.requirements.iter()
            .all(|requirement| activation_table.contains(requirement));
    }
}