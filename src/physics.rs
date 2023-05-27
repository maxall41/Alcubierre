use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::vector;

pub enum FlameColliderType {
    Rectangle((i32,i32)),
    Circle(i32),
    // Capsule(i32,i32)
}

pub struct FlameCollider {
    pub collider_type: FlameColliderType,
    pub sensor: bool,
    pub restitution: f32
}

pub fn pixels_to_physics_units(pixels: i32) -> f32 {
    return pixels as f32 / 50.0
}

pub fn physics_units_to_pixels(units: f32) -> f32 {
    return units * 50.0
}


impl FlameCollider {
    pub fn to_rapier(&self) -> Collider {
        match self.collider_type {
            FlameColliderType::Rectangle((x,y)) => {
                ColliderBuilder::cuboid(pixels_to_physics_units(x) / 2.0,pixels_to_physics_units(y) / 2.0).translation(vector![pixels_to_physics_units(x) / 2.0, pixels_to_physics_units(y) / 2.0]).sensor(self.sensor).restitution(self.restitution).build()
            }
            FlameColliderType::Circle(radius) => {
                ColliderBuilder::ball(pixels_to_physics_units(radius)).sensor(self.sensor).restitution(self.restitution).build()
            }
        }
    }
}