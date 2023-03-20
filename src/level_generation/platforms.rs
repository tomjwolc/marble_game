use super::*;

pub fn spawn_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    default_material: Res<DefaultMaterial>
) {
    /* Create the ground. */
    // commands
    //     .spawn((
    //         Collider::cuboid(100.0, 0.3, 100.0), 
    //         RigidBody::Fixed,
    //         TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0).with_rotation(Quat::from_rotation_x(0.1))),
    //         ActiveEvents::COLLISION_EVENTS,
    //         Friction::coefficient(MARBLE_FRICTION),
    //         InGameEntity,
    //         Jumpy
    //     )).insert(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Box {
    //             min_x: -100.0,
    //             min_y: -0.3,
    //             min_z: -100.0,

    //             max_x: 100.0,
    //             max_y: 0.3,
    //             max_z: 100.0
    //         })),
    //         material: default_material.0.clone(),
    //         transform: Transform::from_xyz(0.0, -1.0, 0.0).with_rotation(Quat::from_rotation_x(0.2)),
    //         ..default()
    //     });

    //     /* Create the ground. */
    //     commands
    //         .spawn((
    //             Collider::cuboid(100.0, 1.0, 100.0), 
    //             RigidBody::Fixed,
    //             TransformBundle::from(Transform::from_xyz(0.0, -7.0, -100.0).with_rotation(Quat::from_rotation_x(0.0))),
    //             ActiveEvents::COLLISION_EVENTS,
    //             Friction::coefficient(MARBLE_FRICTION),
    //             InGameEntity,
    //             Jumpy
    //         )).insert(PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Box {
    //                 min_x: -100.0,
    //                 min_y: -1.0,
    //                 min_z: -100.0,
    
    //                 max_x: 100.0,
    //                 max_y: 1.0,
    //                 max_z: 100.0
    //             })),
    //             material: default_material.0.clone(),
    //             transform: Transform::from_xyz(0.0, -7.0, -100.0),
    //             ..default()
    //         });


    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 35000.0,
            range: 200.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 50.0, 0.0),
        ..default()
    }, InGameEntity));


    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 35000.0,
            range: 200.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 750.0, 0.0),
        ..default()
    }, InGameEntity));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    // directional 'sun' light
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         first_cascade_far_bound: 4.0,
    //         maximum_distance: 10.0,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });

    commands.spawn((Jumpy, MarbleBundle::new(
        7.0 * MARBLE_RADIUS,
        10000000.0,
        0.0, 0.6,
        Transform::from_xyz(-50.0, 0.0, -140.0),
        &mut meshes, 
        default_material.0.clone(),
        Velocity::zero()
    )));

    commands.spawn((Jumpy, MarbleBundle::new(
        7.0 * MARBLE_RADIUS,
        0.001,
        100.0, 0.6,
        Transform::from_xyz(50.0, 0.0, -140.0),
        &mut meshes, 
        default_material.0.clone(),
        Velocity::zero()
    )));

    // commands.spawn((Jumpy, MarbleBundle::new(
    //     150.0,
    //     1000000000.0,
    //     500.0, 1.0,
    //     Transform::from_xyz(0.0, 300.0, 0.0),
    //     &mut meshes, 
    //     default_material.0.clone(),
    //     Velocity::zero()
    // ).fixed()));

    commands.spawn((Jumpy, MarbleBundle::new(
        250.0,
        1000000000.0,
        1.0, 0.3,
        Transform::from_xyz(600.0, 300.0, 0.0),
        &mut meshes, 
        default_material.0.clone(),
        Velocity::zero()
    ).fixed()));
}