use rapier2d::geometry::{Collider, ColliderBuilder};
use rapier2d::prelude::vector;

pub enum AlcubierreColliderType {
    Rectangle((f32, f32)),
    Circle(f32),
    // Capsule(i32,i32)
}

pub struct AlcubierreCollider {
    pub collider_type: AlcubierreColliderType,
    pub sensor: bool,
    pub restitution: f32,
    pub friction: f32,
    pub user_data: u128,
}

pub fn screen_units_to_physics_units(pixels: f32) -> f32 {
    return pixels / 50.0;
}

pub fn physics_units_to_pixels(units: f32) -> f32 {
    return units * 50.0;
}

impl AlcubierreCollider {
    pub fn to_rapier(&self) -> Collider {
        match self.collider_type {
            AlcubierreColliderType::Rectangle((x, y)) => ColliderBuilder::cuboid(
                screen_units_to_physics_units(x) / 1.75,
                screen_units_to_physics_units(y) / 1.75,
            )
            .translation(vector![0.0, screen_units_to_physics_units(y) / 2.0])
            .sensor(self.sensor)
            .friction(self.friction)
            .restitution(self.restitution)
            .user_data(self.user_data)
            .build(),

            AlcubierreColliderType::Circle(radius) => {
                ColliderBuilder::ball(screen_units_to_physics_units(radius))
                    .sensor(self.sensor)
                    .friction(self.friction)
                    .restitution(self.restitution)
                    .user_data(self.user_data)
                    .build()
            }
        }
    }
}
