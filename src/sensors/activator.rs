use super::*;

/* Simple activators have two parts: the sensor and the physical interactive part.  This system
 updates the interactive part (compresses the button, etc.) */
pub fn simple_activator(
    mut activator_query: Query<(&mut Transform, &Activator), With<InGameEntity>>
) {
    for (
        mut transform, 
        Activator { is_active, variant, .. }
    ) in activator_query.iter_mut() {
        match variant {
            ActivatorVariant::Button { initial_position } => {
                transform.translation = if *is_active {
                    initial_position.clone() + transform.down() * BUTTON_COMPRESS_DEPTH
                } else { initial_position.clone() };
            },
            _ => {}
        }
    }
}

pub fn warp_activator(
    mut activator_query: Query<(&mut Activator, &WarpTo), With<InGameEntity>>,
    pkv_store: Res<PkvStore>
) {
    for (mut activator, warp_to) in activator_query.iter_mut() {
        let WarpTo::File(filename) = warp_to else {
            panic!("Activator component given to entity with WarpTo::Out")
        };

        activator.is_active = if let Ok(completed_levels) = pkv_store.get::<CompletedLevels>(COMPLETED_LEVELS) {
            completed_levels.0.contains(filename)
        } else { false }
    }
}