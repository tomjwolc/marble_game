use super::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        Player, 
        MarbleBundle::new(
            MARBLE_RADIUS,
            MARBLE_MASS,
            FRICTION, RESTITUTION,
            Transform::from_xyz(0.0, 10.0, 0.0),
            &mut meshes, 
            materials.add(StandardMaterial {
                base_color_texture: Some(images.add(uv_debug_texture())),
                base_color: Color::WHITE,
                ..default()
            }),
            Velocity::zero()
        ),
        ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO
        },
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