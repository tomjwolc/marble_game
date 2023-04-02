use super::*;

pub fn spawn_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>
) {
    commands.spawn((
        Player,
        MovableBundle::from_shape(
            "ball".to_owned(), 
            vec![MARBLE_RADIUS], 
            &mut mesh_assets, 
            &Handle::default()
        ).with_properties(
            ColliderMassProperties::Mass(MARBLE_MASS), 
            *MATERIAL_PROPERTIES.get("default_movable").unwrap()
        ).with_pbr_bundle(PbrBundle {
            mesh: mesh_assets.add(shape::UVSphere {
                radius: MARBLE_RADIUS,
                sectors: 40,
                stacks: 40
            }.into()),
            material: default_material.0.clone(),
            ..Default::default()
        }).with_channels(SensorChannel::all()),
        ExternalImpulse {
            impulse: Vec3::ZERO,
            torque_impulse: Vec3::ZERO
        },
        Damping { linear_damping: 0.0, angular_damping: ANGULAR_DAMPING }
    ));

    // commands.spawn((
    //     Player, 
    //     MarbleBundle::new(
    //         MARBLE_RADIUS,
    //         MARBLE_MASS,
    //         MARBLE_FRICTION, MARBLE_RESTITUTION,
    //         Transform::from_xyz(0.0, 0.0, 0.0),
    //         &mut meshes, 
    //         default_material.0.clone(),
    //         Velocity::zero()
    //     ),
    //     ExternalImpulse {
    //         impulse: Vec3::ZERO,
    //         torque_impulse: Vec3::ZERO
    //     },
    //     Damping { linear_damping: 0.0, angular_damping: ANGULAR_DAMPING },
    //     SensorChannel::all()
    // ));

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