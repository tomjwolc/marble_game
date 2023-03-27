use super::*;

#[derive(Bundle)]
pub struct WarpBundle {
    pub friction: Friction,
    pub restitution: Restitution,

    pub activatable: Activatable,
    pub warp_to: WarpTo,

    pub collider: Collider,
    pub pbr_bundle: PbrBundle,
    pub in_game_entity: InGameEntity,
    pub jumpy: Jumpy
}

impl Default for WarpBundle {
    fn default() -> Self {
        Self {
            friction: Friction::coefficient(WARP_FRICTION),
            restitution: Restitution::coefficient(WARP_RESTITUTION),
            activatable: Activatable { requirements: Vec::new(), is_active: true },
            warp_to: WarpTo::Out,
            collider: Collider::cylinder(0.25 * SCALE, 0.75 * SCALE),
            pbr_bundle: PbrBundle::default(),
            in_game_entity: InGameEntity,
            jumpy: Jumpy
        }
    }
}