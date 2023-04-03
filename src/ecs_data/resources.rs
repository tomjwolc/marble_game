use bevy::gltf::Gltf;

use super::*;

#[derive(Resource, Clone)]
pub struct DefaultMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct CanJump(pub bool);

#[derive(Resource)]
pub struct ActivationTable(pub Vec<bool>);

#[derive(Resource)]
pub struct LoadedGlbData(pub Vec<LoadedGlbObject>);

#[derive(Clone)]
pub struct LoadedGlbObject {
    pub object_type: GltfObjectType,
    pub collider: Collider,
    pub transform: Transform,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>
}

#[derive(Resource)]
pub struct LevelStack(Vec<Level>);

pub struct Level {
    pub handle: Option<Handle<Gltf>>,
    pub file_name: String
}

// It is assumed that LevelStack will always have at least one element in it
impl LevelStack {
    pub fn pop(&mut self) -> Level {
        self.0.pop().unwrap()
    }

    pub fn push(&mut self, level: Level) {
        self.0.push(level);
    }

    pub fn from_level(file_name: &'static str) -> Self {
        Self(vec![Level { handle: None, file_name: file_name.to_owned() }])
    }

    pub fn warp(&mut self, warp_to: &WarpTo) {
        match warp_to {
            WarpTo::File(file_name) => self.push(Level { 
                handle: None, 
                file_name: file_name.clone()
            }), 
            WarpTo::Out => {
                self.pop();
            }
        }
    }

    pub fn get_current_level(&self) -> &Level {
        self.0.last().unwrap()
    }

    pub fn get_current_level_mut(&mut self) -> &mut Level {
        self.0.last_mut().unwrap()
    }
}