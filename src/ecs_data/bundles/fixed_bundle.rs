use super::*;

#[derive(Bundle)]
pub struct FixedBundle {
    pbr_bundle: PbrBundle,
    collider: Collider,
    friction: Friction,
    restitution: Restitution,
    in_game_entity: InGameEntity,
    jumpy: Jumpy
}

impl FixedBundle {
    pub fn with_properties(mut self, material_properties: MaterialProperties) -> Self {
        self.friction = material_properties.friction;
        self.restitution = material_properties.restitution;

        self
    }
}

impl FromShape for FixedBundle {
    fn from_collider(collider: Collider) -> Self {
        Self {
            collider,
            ..Default::default()
        }
    }

    fn with_transform(mut self, transform: Transform) -> Self {
        self.pbr_bundle.transform = transform;

        self
    }

    fn with_pbr_bundle(mut self, pbr_bundle: PbrBundle) -> Self {
        self.pbr_bundle = pbr_bundle;

        self
    }
}

impl Default for FixedBundle {
    fn default() -> Self {
        Self {
            pbr_bundle: PbrBundle::default(),
            collider: Collider::default(),
            friction: MOVABLE_FRICTION,
            restitution: MOVABLE_RESTITUTION,
            in_game_entity: InGameEntity,
            jumpy: Jumpy
        }
    }
}