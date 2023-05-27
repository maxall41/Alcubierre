pub fn pixels_to_physics_units(pixels: i32) -> f32 {
    return pixels as f32 / 50.0
}

pub fn physics_units_to_pixels(units: f32) -> f32 {
    return units * 50.0
}