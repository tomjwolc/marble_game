use super::*;

pub fn spawn_lighting(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: BUTTON_COLOR,
        brightness: 0.05,
    });
    
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        point_light: PointLight {
            intensity: 10000000.0, 
            color: BUTTON_COLOR,
            shadows_enabled: true,
            range: 10000.0,
            radius: 1.0,
            shadow_depth_bias: 0.000001,
            shadow_normal_bias: 1.0,
            ..default()
        },
        ..default()
    });
}