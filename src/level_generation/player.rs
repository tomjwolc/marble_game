use super::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>
) {
    commands.spawn((
        Player, 
        MarbleBundle::new(
            MARBLE_RADIUS,
            MARBLE_MASS,
            MARBLE_FRICTION, MARBLE_RESTITUTION,
            Transform::from_xyz(0.0, 10.0, 0.0),
            &mut meshes, 
            default_material.0.clone(),
            Velocity::zero()
        ),
        ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO
        }
    ));

    commands.spawn((
        PlayerSensor,
        InGameEntity,
        Sensor,
        Collider::ball(0.9 * MARBLE_RADIUS),
        RigidBody::Dynamic,
        LockedAxes::TRANSLATION_LOCKED,
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0))
    ));
}

pub fn reset_camera(mut camera_transform_query: Query<&mut Transform, With<Camera>>) {
    *camera_transform_query.single_mut() = Transform::
        from_xyz(0.0, 0.0, -4.0)
        .looking_at(Vec3::ZERO, Vec3::Y).with_scale(SCALE * Vec3::ONE)
}