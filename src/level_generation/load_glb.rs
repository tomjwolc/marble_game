// use bevy::scene::SceneInstance;

use bevy::gltf::{GltfMesh, GltfNode};

use super::*;

pub fn load_glb_asset(
    asset_server: Res<AssetServer>,
    mut next_level: ResMut<NextLevel>
) {
    let gltf = asset_server.load(format!("levels/{}.glb", next_level.file_name).as_str());
    next_level.handle = Some(gltf.clone());
}

pub fn load_glb(
    mut commands: Commands,
    next_level: Res<NextLevel>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    mesh_assets: Res<Assets<Mesh>>
) {
    if let Some(gltf) = gltf_assets.get(&next_level.handle.as_ref().unwrap()) {
        commands.spawn((SceneBundle {
            scene: gltf.scenes[0].clone(),
            transform: Transform::from_scale(SCALE * Vec3::ONE),
            ..Default::default()
        }, InGameEntity));

        for node_handle in gltf.nodes.iter() {
            load_gltf_node_collider(
                &mut commands,
                gltf_node_assets
                    .get( node_handle )
                    .expect("Could not find node in gltf_node_assets"), 
                &gltf_mesh_assets, 
                &mesh_assets
            );
        }
    }
}

fn load_gltf_node_collider(
    commands: &mut Commands,
    node: &GltfNode,
    gltf_mesh_assets: &Res<Assets<GltfMesh>>,
    mesh_assets: &Res<Assets<Mesh>>
) {
    let mut transform = node.transform;

    transform = transform.with_scale(SCALE * Vec3::ONE);
    transform.translation *= SCALE;

    for child_node in node.children.iter() {
        load_gltf_node_collider(
            commands,
            child_node, 
            gltf_mesh_assets, 
            mesh_assets
        );
    }

    ignore_extract!(
        Some( gltf_mesh_handle ) = &node.mesh;
        Some( gltf_mesh ) = gltf_mesh_assets.get( &gltf_mesh_handle )
    );

    for gltf_primitive in gltf_mesh.primitives.iter() {
        if let Some( mesh ) = mesh_assets.get(&gltf_primitive.mesh) {
            commands.spawn((
                Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                    .expect("Could not create collider from bevy mesh"),
                ActiveEvents::COLLISION_EVENTS,
                TransformBundle::from_transform(transform),
                InGameEntity,
                Jumpy
            ));
        }
    }
}