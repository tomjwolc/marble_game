// use bevy::scene::SceneInstance;

use bevy::gltf::{GltfMesh, GltfNode, GltfPrimitive};
use serde::{Serialize, Deserialize};

use super::*;

pub fn load_glb_asset(
    asset_server: Res<AssetServer>,
    mut level_stack: ResMut<LevelStack>
) {
    let gltf = asset_server.load(
        format!("levels/{}.glb", level_stack.get_current_level().file_name).as_str()
    );
    level_stack.get_current_level_mut().handle = Some(gltf.clone());
}

pub fn load_glb(
    mut commands: Commands,
    level_stack: Res<LevelStack>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    mesh_assets: Res<Assets<Mesh>>
) {
    if let Some(gltf) = gltf_assets.get(&level_stack.get_current_level().handle.as_ref().unwrap()) {
        for node_handle in gltf.nodes.iter() {
            //     Continues if the node asset does not exist, the node has no mesh, 
            // or the mesh asset does not exist
            extract!(continue; 
                Some( node ) = gltf_node_assets.get(node_handle);
                Some( gltf_mesh_handle ) = &node.mesh;
                Some( gltf_mesh ) = gltf_mesh_assets.get(gltf_mesh_handle)
            );

            // println!("{}",  if let Some(extras) = &node.extras { extras.value.as_str() } else { "{}" });

            let extras_data: ExtrasData = serde_json::from_str(
                if let Some(extras) = &node.extras { extras.value.as_str() } else { "{}" }
            ).unwrap();

            // println!("{:?}", GltfObjectType::from(extras_data.clone()));

            let mut transform = node.transform.with_scale(SCALE * Vec3::ONE);
            transform.translation *= SCALE;

            commands.spawn((
                InGameEntity, 
                Visibility::Visible,
                ComputedVisibility::default(),
                TransformBundle::from(transform)
            )).with_children(|parent| {
                GltfObjectType::from(extras_data).spawn_bundles(
                    parent, 
                    &gltf_mesh.primitives, 
                    &mesh_assets
                );
            });
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExtrasData {
    file_name: Option<String>,
    //     Meant to be vec, but blender doesn't support variable length arrays
    // for custom properties, so String in form "1,3,4"
    activation_requirements: Option<String>,
    activator_type: Option<String>,
    activator_id: Option<usize>,
    on_activate: Option<String>,
    planet_radius: Option<f32>,
    // The i32 means nothing, the field just checks that it is a respawn_detector
    respawn_detector: Option<i32>
}

#[derive(Debug)]
enum GltfObjectType {
    Warp(WarpTo, Activatable),
    // Button(Activator),
    RespawnDetector,
    // Planet(f32),
    Object
}

impl GltfObjectType {
    fn spawn_bundles(&self, 
        parent: &mut ChildBuilder, 
        primitives: &Vec<GltfPrimitive>, 
        mesh_assets: &Assets<Mesh>
    ) {
        match self {
            GltfObjectType::Warp(warp_to, activatable) => {
                let Some( mesh ) = mesh_assets.get(&primitives[0].mesh) else { return };

                parent.spawn(WarpBundle {
                    collider: Collider::from_bevy_mesh(
                        mesh, 
                        &ComputedColliderShape::TriMesh
                    ).expect("Could not create collider from warp mesh"),
                    pbr_bundle: PbrBundle {
                        mesh: primitives[0].mesh.clone(),
                        material: primitives[0].material.clone().expect("Warp primitive didn't have material"),
                        ..Default::default()
                    },
                    warp_to: warp_to.clone(),
                    activatable: activatable.clone(),
                    ..Default::default()
                });
            },
            GltfObjectType::RespawnDetector => {
                let Some( mesh ) = mesh_assets.get(&primitives[0].mesh) else { return };

                parent.spawn((
                    Collider::from_bevy_mesh(
                        mesh, 
                        &ComputedColliderShape::TriMesh
                    ).expect("Could not create collider from warp mesh"),
                    RigidBody::Dynamic,
                    LockedAxes::TRANSLATION_LOCKED,
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
                    Sensor,
                    SensorChannel::Respawn
                ));
            },
            _ => {
                for primitive in primitives.iter() {
                    let Some( mesh ) = mesh_assets.get(&primitive.mesh) else { continue };

                    parent.spawn((
                        PbrBundle {
                            mesh: primitive.mesh.clone(),
                            material: primitive.material
                                .as_ref()
                                .expect("Could not find material in primitive")
                                .clone(),
                            ..Default::default()
                        },
                        Collider::from_bevy_mesh(
                            mesh, 
                            &ComputedColliderShape::TriMesh
                        ).expect("Could not create collider from bevy mesh"),
                        Jumpy
                    ));
                }
            }
        }
    }
}

impl From<ExtrasData> for GltfObjectType {
    fn from(value: ExtrasData) -> Self {
        match value {
            ExtrasData { 
                file_name: Some(file_name), 
                activation_requirements: Some(activation_requirements), 
                .. 
            } => GltfObjectType::Warp(
                if file_name.len() == 0 { WarpTo::Out } else { WarpTo::File(file_name) }, 
                Activatable {
                    requirements: if activation_requirements.len() == 0 {
                        Vec::new()
                    } else {
                        activation_requirements
                            .split(",").into_iter()
                            .map(|activator_id|  activator_id.parse::<usize>().expect("Could not parse int"))
                            .collect()
                    },
                    is_active: activation_requirements.len() == 0
                }
            ), ExtrasData {
                respawn_detector: Some(_),
                ..
            } => GltfObjectType::RespawnDetector,
            _ => GltfObjectType::Object
        }
    }
}