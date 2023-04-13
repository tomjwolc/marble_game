use super::*;

pub fn activate_activatables(
    mut activatable_query: Query<&mut Activatable, With<InGameEntity>>,
    activation_table: Res<ActivationTable>
) {
    for mut activatable in activatable_query.iter_mut() {
        activatable.is_active = activatable.requirements.iter()
            .all(|&requirement| activation_table.0[requirement]);
    }
}

/* Activators have two parts: the sensor and the physical interactive part.  This system
 updates the interactive part (compresses the button, etc.) */
pub fn engage_activator(
    mut activator_query: Query<(&mut Transform, &Activator, &ActivatorType), With<InGameEntity>>,
    activation_table: Res<ActivationTable>
) {
    for (mut transform, Activator(id), activator_type) in activator_query.iter_mut() {
        let is_active = activation_table.0[*id];

        match activator_type {
            ActivatorType::Button { initial_position } => {
                transform.translation = if is_active {
                    initial_position.clone() + transform.down() * BUTTON_COMPRESS_DEPTH
                } else { initial_position.clone() };
            },
            _ => {}
        }
    }
}

pub fn warp_activation(
    mut activatable_query: Query<(&Activatable, &mut SensorChannel), (With<Sensor>, With<WarpTo>, With<InGameEntity>)>
) {
    for (
        &Activatable { is_active, .. }, 
        mut sensor_channel
    ) in activatable_query.iter_mut() {
        *sensor_channel = if is_active { SensorChannel::Warp } else { SensorChannel::None };
    }
}