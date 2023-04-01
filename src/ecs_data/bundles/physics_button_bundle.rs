use super::*;

#[derive(Bundle)]
pub struct PhysicsButtonBundle {
    pbr_bundle: PbrBundle,
    collider: Collider,
    rigid_body: RigidBody,
    friction: Friction,
    restitution: Restitution,
    activator: Activator,
    activator_type: ActivatorType,
    in_game_entity: InGameEntity,
    jumpy: Jumpy
}


impl PhysicsButtonBundle {
    pub fn new(transform: Transform, material: Handle<StandardMaterial>, mesh_assets: &mut Assets<Mesh>, id: usize) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: mesh_assets.add(shape::Cylinder { 
                    radius: BUTTON_RADIUS,
                    height: BUTTON_HEIGHT,
                    ..Default::default() 
                }.into()),
                material,
                transform,
                ..Default::default()
            },
            activator: Activator(id),
            activator_type: ActivatorType::Button { initial_position: transform.translation },
            ..Default::default()
        }
    }
}

impl Default for PhysicsButtonBundle {
    fn default() -> Self {
        Self {
            pbr_bundle: PbrBundle::default(),
            collider: Collider::cylinder(BUTTON_HEIGHT / 2.0, BUTTON_RADIUS),
            rigid_body: RigidBody::KinematicPositionBased,
            friction: DEFAULT_MATERIAL_PROPERTIES.friction,
            restitution: DEFAULT_MATERIAL_PROPERTIES.restitution,
            activator: Activator(0),
            activator_type: ActivatorType::Button { initial_position: Vec3::ZERO },
            in_game_entity: InGameEntity,
            jumpy: Jumpy
        }
    }
}