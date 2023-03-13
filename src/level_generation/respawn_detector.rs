use super::*;

#[derive(Component)]
pub struct RespawnSensor;

pub fn setup_respawn_detector(
    mut commands: Commands
) {
    commands
        .spawn((
            Collider::cuboid(10000.0, 100.0, 10000.0), 
            RigidBody::Dynamic,
            LockedAxes::TRANSLATION_LOCKED,
            TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)),
            InGameEntity,
            Sensor,
            RespawnSensor
        ));
}

pub fn check_detection(
    rapier_context: Res<RapierContext>,
    respawn_sensor_entity_query: Query<Entity, With<RespawnSensor>>,
    player_entity_query: Query<Entity, (With<Player>, Without<RespawnSensor>)>,
    mut states: ResMut<NextState<AppState>>,
    mut menu_scheduler: ResMut<MenuScheduler>
) {
    let Ok(sensor_entity) = respawn_sensor_entity_query.get_single() else {
        return
    };

    let Ok(player_entity) = player_entity_query.get_single() else {
        return
    };
    
    for (collider1, collider2, is_intersecting) in rapier_context.intersections_with(sensor_entity) {
        if is_intersecting && (player_entity == collider1 || player_entity == collider2) {
            menu_scheduler.set_menu_type(MenuType::DeathScreen);
            states.set(AppState::OverlayMenu);
        }
    }
}