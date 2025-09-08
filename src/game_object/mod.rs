pub mod behaviour;
pub mod world;

extern crate sdl3;

use crate::serialization::{Action, AssetId};
use crate::game_object::behaviour::{Behaviour, BehaviourParameter};
use crate::math::bounds::Bounds;
use crate::math::vector2::Vector2;
use sdl3::pixels::Color;
use sdl3::render::FRect;

pub type PhysicsVector = Vector2<f32>;

pub struct Drawable {
    pub z: i32,
    pub color: Color,
    pub texture: Option<AssetId>, // index of the texture
    pub tint_texture: bool,       // if the texture should be tinted by the color
}

impl Default for Drawable {
    fn default() -> Self {
        Self {
            z: i32::default(),
            color: Color::WHITE,
            texture: None,
            tint_texture: bool::default(),
        }
    }
}

pub struct ActionHandler {}

pub struct GameObject {
    pub id: i32,
    pub bounds: FRect,

    pub drawable: Option<Drawable>,
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
            action_handler: None,
            behaviours: Vec::new(),
        }
    }

    pub fn tick(&mut self, delta_t: f64, actions: Action, other_bounds: &Vec<(i32, FRect)>) {
        let behaviours = &mut self.behaviours;
        let mut bounds = self.bounds;
		let mut collisions = Vec::new();
		let mut force = None;
		let mut impulse = None;

        for i in 0..behaviours.len() {
            let behaviour = behaviours[i].as_mut();

            let result = behaviour.tick(
                BehaviourParameter {
                    id: self.id,
                    bounds,
                    actions,
                    other_bounds,
					collisions: &collisions,
					force,
					impulse,
                },
                delta_t,
            );

            if let Some(b) = result.bounds {
                bounds = b
            }

			if let Some(c) = result.collisions {
				collisions = c
			}

			force = result.force;
			impulse = result.impulse;
        }

        self.bounds = bounds;
    }
}
