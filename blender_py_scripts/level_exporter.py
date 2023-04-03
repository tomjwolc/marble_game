import bpy
import os

overwrite_shapes = False

# Get the filepath of the current Blender file
filepath = bpy.data.filepath
filename = os.path.splitext(os.path.basename(filepath))[0]

# !!! Change to the filepath on your computer !!!
output_filepath = "desktop/rust_files/marble_game/assets/levels/" + filename + ".glb"

# Get all the objects in the scene
scene_objects = bpy.context.scene.objects

# Iterate through each object in the scene
for obj in scene_objects:
    if obj.type != "MESH":
        continue
    
    obj["collider_dimensions"] = [
        obj.dimensions[0],
        obj.dimensions[2],
        obj.dimensions[1]
    ];
    
    # Apllies Scale to all objects
    bpy.context.view_layer.objects.active = obj
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    
    if not "shape" in obj or overwrite_shapes:
        primitive_shape = "complex"
        num_vertices = len(obj.data.vertices)
        num_faces = len(obj.data.polygons)
        
        if num_vertices == 8 and num_faces == 6:
            primitive_shape = "cuboid"
        elif num_vertices > 100 and num_faces > 100 and abs(obj.dimensions[0] - obj.dimensions[1]) < 0.1 and abs(obj.dimensions[0] - obj.dimensions[2]) < 0.1:
            primitive_shape = "ball"
        
        obj["shape"] = primitive_shape
        
    print("Updated \"" + obj.name + "\"")
    print("    shape: " + obj["shape"])
    print("    collider_dimensions: { x: " +
        str(obj.dimensions[0]) + ", y: " +
        str(obj.dimensions[2]) + ", z: " + 
        str(obj.dimensions[1])
    + " }")

# Export the scene as a glTF binary file with the name of the Blender file
bpy.ops.export_scene.gltf(
    filepath=output_filepath, 
    check_existing=False,
    export_format='GLB', 
    export_extras=True
)

print("Exported " + output_filepath)