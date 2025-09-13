pub mod behaviour;
pub mod world;
pub mod drawable;

extern crate sdl3;

use crate::serialization::AssetBounds;
use crate::game_object::behaviour::{BehaviourParameter, BehaviourType};
use crate::math::bounds::Bounds;
use crate::math::vector2::Vector2;
use sdl3::render::FRect;
use serde::{Deserialize, Serialize};
use crate::actions::Action;
use crate::game_object::drawable::{DrawLayer, Drawable};

pub type PhysicsVector = Vector2<f32>;

pub type ObjectMask = u32;
pub type BoundInfo = (i32, FRect, ObjectMask);
pub type CollisionInfo = BoundInfo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameObject {
    pub id: i32,
	#[serde(with = "AssetBounds")]
    pub bounds: FRect,
	#[serde(default)]
	pub mask: ObjectMask,
    pub drawable: Option<Drawable>,
    pub behaviours: Vec<BehaviourType>,
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
			mask: ObjectMask::default(),
            drawable: Some(Drawable::default()),
            behaviours: Vec::new(),
        }
    }

    pub fn tick(&mut self, delta_t: f64, world_bounds: FRect, actions: Action, other_bounds: &Vec<BoundInfo>) {
        let behaviours = &mut self.behaviours;
        let mut bounds = self.bounds;
		let mut collisions = Vec::new();
		let mut force = None;
		let mut impulse = None;

        for i in 0..behaviours.len() {
            let behaviour = &mut behaviours[i];

            let result = behaviour.tick(
                BehaviourParameter {
                    id: self.id,
                    bounds,
                    actions,
					world_bounds,
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
