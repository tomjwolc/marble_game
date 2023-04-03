# Using Blender for Creating Levels

**Workflow:** Run blender_scripts/level_exporter.py on your blender file after editing the file_path for your specific system

## Sensor
**Required custom properties**
```rust
sensor_channel: String
```
**Optional custom properties**
```rust
gravity_direction: [f32; 3] // for gravity sensors
```