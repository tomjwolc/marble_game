pub use super::*;

pub fn reorient_gravity(
    mut gravity_query: Query<(&Transform, &ColliderMassProperties, &mut Gravity)>,
    gravity_well_query: Query<(&Transform, &ColliderMassProperties), Without<NotGravityWell>>
) {
    for (
        entity_transform, 
        collider_mass_properties,
        mut gravity_component
    ) in gravity_query.iter_mut() {
        let Gravity(gravity, is_in_gravity_change_sensor) = &mut *gravity_component;
        let ColliderMassProperties::Mass(entity_mass) = collider_mass_properties else { continue };

        // filter out entities in gravity sensors, since they aren't affected by gravity wells
        if !(*is_in_gravity_change_sensor) {
            *gravity = Vec3::ZERO;

            for (
                well_transform, 
                well_collider_mass_properties
            ) in gravity_well_query.iter() {
                let distance_vector = well_transform.translation - entity_transform.translation;
                let ColliderMassProperties::Mass(well_mass) = well_collider_mass_properties else { continue };

                // Fg = Gm1m2/r^2
                *gravity += if distance_vector.length() > 0.0 {
                    GRAVITATIONAL_CONSTANT * well_mass * entity_mass * distance_vector.normalize() / 
                    distance_vector.length_squared()
                } else {
                    Vec3::ZERO
                }
            }
        }
    }
}

pub fn apply_gravitational_force(
    mut gravity_query: Query<(&Gravity, &RigidBody, &mut ExternalForce)>
) {
    for (Gravity(gravity, _), rigid_body, mut external_force) in gravity_query.iter_mut() {
        if let RigidBody::Dynamic = rigid_body {
            external_force.force = *gravity;
        }
    }
}