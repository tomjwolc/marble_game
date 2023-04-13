import bpy
import os

overwrite_shapes = False

# pass in obj, assumes obj.type == "MESH"
def guess_shape(obj):
    # tries searching the name first
    if "cube" in obj.name.lower():
        return "cuboid"
    elif "cylinder" in obj.name.lower():
        return "cylinder"
    elif "sphere" in obj.name.lower():
        return "ball"
    
    if not hasattr(obj, "data"):
        return "complex"

    num_vertices = len(obj.data.vertices)
    num_faces = len(obj.data.polygons)
    (x, y, z) = obj.dimensions.to_tuple()

    print("# vertices: ", num_vertices)
    print("# faces: ", num_vertices)
    print("dimensions: ", (x, y, z))

    if num_vertices == 8 and num_faces == 6:
        return "cuboid"
    elif num_vertices > 100 and num_faces > 100 and abs(x - y) < 0.01 and abs(x - z) < 0.01:
        return "ball"
    else:
        return "complex"

# Get the filepath of the current Blender file
filepath = bpy.data.filepath
filename = os.path.splitext(os.path.basename(filepath))[0]

#print(filepath)
#print(bpy.path.abspath)

# !!! Change to the filepath on your computer !!!
output_filepath = "Desktop/rust_files/marble_game/assets/levels/" + filename + ".glb"

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

    print(obj.name + " is a " + guess_shape(obj))
    
    # Apllies Scale to all objects
    bpy.context.view_layer.objects.active = obj
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    
    if (not "shape" in obj) or overwrite_shapes:
        obj["shape"] = guess_shape(obj)

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