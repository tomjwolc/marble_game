pub use super::*;

mod marble_bundle;
pub use marble_bundle::*;

#[derive(Bundle)]
pub struct GravityBundle {
    gravity: Gravity,
    grivity_scale: GravityScale,
    force: ExternalForce,
    mass: ColliderMassProperties
}

impl GravityBundle {
    pub fn from_mass(mass: f32) -> Self {
        Self { 
            gravity: Gravity(Vec3::ZERO, false), 
            grivity_scale: GravityScale(0.0),
            force: ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO } ,
            mass: ColliderMassProperties::Mass(mass)
        }
    }
}

impl Default for GravityBundle {
    fn default() -> Self {
        Self { 
            gravity: Gravity(Vec3::ZERO, false), 
            grivity_scale: GravityScale(0.0),
            force: ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO } ,
            mass: ColliderMassProperties::Mass(1.0)
        }
    }
}