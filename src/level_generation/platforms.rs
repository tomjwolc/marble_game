use super::*;

pub fn spawn_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    /* Create the ground. */
    commands
        .spawn((
            Collider::cuboid(100.0, 0.3, 100.0), 
            RigidBody::Fixed,
            TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0).with_rotation(Quat::from_rotation_x(0.1))),
            ActiveEvents::COLLISION_EVENTS,
            Friction::coefficient(FRICTION),
            InGameEntity,
            Jumpy
        )).insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -100.0,
                min_y: -0.3,
                min_z: -100.0,

                max_x: 100.0,
                max_y: 0.3,
                max_z: 100.0
            })),
            material: debug_material.clone(),
            transform: Transform::from_xyz(0.0, -2.0, 0.0).with_rotation(Quat::from_rotation_x(0.1)),
            ..default()
        });

        /* Create the ground. */
        commands
            .spawn((
                Collider::cuboid(100.0, 1.0, 100.0), 
                RigidBody::Fixed,
                TransformBundle::from(Transform::from_xyz(100.0, -7.0, 0.0).with_rotation(Quat::from_rotation_x(0.0))),
                ActiveEvents::COLLISION_EVENTS,
                Friction::coefficient(FRICTION),
                InGameEntity,
                Jumpy
            )).insert(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -100.0,
                    min_y: -1.0,
                    min_z: -100.0,
    
                    max_x: 100.0,
                    max_y: 1.0,
                    max_z: 100.0
                })),
                material: debug_material,
                transform: Transform::from_xyz(100.0, -7.0, 0.0),
                ..default()
            });


    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    }, InGameEntity));
}