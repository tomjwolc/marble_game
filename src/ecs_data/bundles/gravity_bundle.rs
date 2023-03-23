use super::*;

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
            mass: ColliderMassProperties::Mass(mass),
            ..Default::default()
        }
    }

    pub fn with_gravity(mut self, gravity: Gravity) -> Self {
        self.gravity = gravity;

        self
    }
}

impl Default for GravityBundle {
    fn default() -> Self {
        Self { 
            gravity: Gravity(-GRAVITY * Vec3::Y, GravityType::Constant), 
            grivity_scale: GravityScale(0.0),
            force: ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO } ,
            mass: ColliderMassProperties::Mass(1.0)
        }
    }
}