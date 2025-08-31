pub mod world;

extern crate sdl3;

use crate::math::bounds::Bounds;
use crate::math::vector2::Vector2;
use sdl3::pixels::Color;
use sdl3::render::{FRect};

pub type PhysicsVector = Vector2<f32>;

pub struct Drawable {
    pub z: i32,
    pub color: Color,
    pub texture: usize, // index of the texture
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicsFrame {
    pub speed: PhysicsVector,
    pub acceleration: PhysicsVector,
    pub mass: f32,
    pub friction_coefficient: f32,
    pub resistance_coefficient: f32, // air or water resistance
}

#[derive(Debug, Default, Clone)]
pub struct PhysicsBody {
    pub current_frame: PhysicsFrame,
    pub next_frame: PhysicsFrame,
}

impl PhysicsBody {
    pub fn apply_force(&mut self, force: &PhysicsVector) {

    }

    pub fn apply(&mut self) {
        self.current_frame = self.next_frame;
    }
}

pub struct ActionHandler {

}

pub struct GameObject {
    pub id: i32,
    pub bounds: FRect,

    pub drawable: Option<Drawable>,
    pub physics_body: Option<PhysicsBody>,
    pub action_handler: Option<ActionHandler>
}

impl GameObject {
    pub fn tick(&mut self, delta_t: u64) {}
}
