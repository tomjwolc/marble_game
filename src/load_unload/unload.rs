use super::*;

pub fn despawn_all_game_entities(
    entities_query: Query<Entity, Or<(&InGameEntity, &SoftUnloadedData)>>,
    mut level_stack: ResMut<LevelStack>,
    mut commands: Commands
) {
    if DEBUG_MENUS || DEBUG_LOAD { println!("Complete unload") };

    *level_stack = LevelStack::initial_stack();

    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_loaded_game_entities(
    entities_query: Query<Entity, With<InGameEntity>>,
    mut commands: Commands
) {
    if DEBUG_MENUS || DEBUG_LOAD { println!("Hard unload") };

    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn soft_despawn_game_entities(
    mut entities_query: Query<(Entity, 
        Option<&mut Visibility>, 
        Option<&Collider>, 
        Option<&RigidBody>,
        Option<&AddTimerForReload> 
    ), With<InGameEntity>>, 
    level_stack: Res<LevelStack>,
    mut commands: Commands
) {
    if DEBUG_MENUS || DEBUG_LOAD { println!("Soft unload") };

    for (
        entity, 
        visibility_option, 
        collider_option,
        rigid_body_option,
        add_timer_option
    ) in entities_query.iter_mut() {
        let entity_commands = &mut commands.entity(entity);

        entity_commands.remove::<InGameEntity>();
        entity_commands.insert(SoftUnloadedData {
            id: level_stack.len() - 1,
            collider_option: collider_option.map(|collider| collider.clone()),
            rigid_body_option: rigid_body_option.map(|rigid_body| rigid_body.clone()),
            give_lifetime: if let Some(_) = add_timer_option { 
                Some(RELOAD_WARP_DISABLE_DURATION) 
            } else { 
                None
            }
        });

        if let Some(mut visibility) = visibility_option {
            *visibility = Visibility::Hidden;
        }

        if collider_option.is_some() {
            commands.entity(entity).remove::<Collider>();
        }

        if rigid_body_option.is_some() {
            commands.entity(entity).remove::<RigidBody>();
        }
    }
}