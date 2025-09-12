use crate::serialization::AssetBounds;
use crate::serialization::Action;
use crate::game_object::{PhysicsVector};
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;
use sdl3::render::FRect;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ControllableBehaviour {
	#[serde(with = "AssetBounds")]
    bounds: FRect,
    speed: f32,
    run_speed: f32,
	jumping: f32,
	can_jump: bool,
	velocity: PhysicsVector,
	acceleration: PhysicsVector,
}

impl ControllableBehaviour {
    pub fn new(bounds: FRect, speed: f32, run_speed: f32) -> Self {
        Self {
            bounds,
            speed,
            run_speed,
			jumping: 0.0,
			can_jump: true,
			velocity: PhysicsVector::default(),
			acceleration: PhysicsVector::default(),
        }
    }
}

impl Behaviour for ControllableBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: f64) -> BehaviourResult {
        let sec = delta_t as f32;
		let bounds = params.bounds;
        let actions = params.actions;
		let mut force = PhysicsVector::default();
		let mut impulse = PhysicsVector::default();

        let speed = if actions.contains(Action::Sprint) {
            self.run_speed
        } else {
            self.speed
        };

		let mut in_air = true;

		for i in 0..params.collisions.len() {
			let (_, collision) = params.collisions[i];

			if collision.w > collision.h && collision.top() > bounds.center().y {
				in_air = false;

				break;
			}
		}

		if !in_air && actions.contains(Action::Jump) && self.jumping == 0.0 {
			self.jumping = 0.3;	// 0.3 sec cooldown for jumping; this is just a dirty fix for multi-jumps
			impulse += PhysicsVector { x: 0.0, y: -self.run_speed };
		}

		self.jumping = f32::max(0.0, self.jumping - sec);

        // if actions.contains(Action::MoveUp) {
        //     position.y -= speed * sec;
        // } else if actions.contains(Action::MoveDown) {
        //     position.y += speed * sec;
        // }

        if actions.contains(Action::MoveLeft) {
			force += PhysicsVector { x: -speed, y: 0.0 };
        } else if actions.contains(Action::MoveRight) {
			force += PhysicsVector { x: speed, y: 0.0 };
        }

        BehaviourResult {
            bounds: None,
            collisions: None,
			force: Some(force),
			impulse: Some(impulse),
        }
    }
}
