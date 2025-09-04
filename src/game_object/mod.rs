pub mod world;
pub mod behaviour;

extern crate sdl3;

use crate::math::bounds::Bounds;
use crate::math::vector2::Vector2;
use sdl3::pixels::Color;
use sdl3::render::{FRect};
use crate::game_data::{Action, AssetId};
use crate::game_object::behaviour::{Behaviour, BehaviourParameter};

pub type PhysicsVector = Vector2<f32>;

pub struct Drawable {
    pub z: i32,
    pub color: Color,
    pub texture: Option<AssetId>, // index of the texture
	pub tint_texture: bool, // if the texture should be tinted by the color
}

impl Default for Drawable {
	fn default() -> Self {
		Self {
			z: i32::default(),
			color: Color::WHITE,
			texture: None,
			tint_texture: bool::default()
		}
	}
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
    pub fn apply_force(&mut self, game_object: &GameObject, force: &PhysicsVector) {}

    pub fn apply(&self, game_object: &mut GameObject, delta_t: u64) {

	}
}

pub struct ActionHandler {}

pub struct GameObject {
    pub id: i32,
    pub bounds: FRect,

    pub drawable: Option<Drawable>,
    pub physics_body: Option<PhysicsBody>,
    pub action_handler: Option<ActionHandler>,
    pub behaviours: Vec<Box<dyn Behaviour>>,
}

impl GameObject {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            bounds: FRect {
                x: f32::default(),
                y: f32::default(),
                w: f32::default(),
                h: f32::default(),
            },
            drawable: Some(Drawable::default()),
            physics_body: None,
            action_handler: None,
            behaviours: Vec::new(),
        }
    }

    pub fn tick(&mut self, delta_t: f64, actions: Action, other_bounds: &Vec<(i32, FRect)>) {
        let behaviours = &mut self.behaviours;
        let mut bounds = self.bounds;

        for i in 0..behaviours.len() {
            let behaviour = behaviours[i].as_mut();

            let result = behaviour.tick(BehaviourParameter { id: self.id, bounds, actions, other_bounds }, delta_t);

            if let Some(b) = result.bounds {
                bounds = b
            }
        }

		self.bounds = bounds;
    }
}
