use crate::game_object::{PhysicsVector};
use crate::game_object::behaviour::{Behaviour, BehaviourParameter, BehaviourResult};
use crate::math::bounds::Bounds;
use crate::math::VectorOps;
use serde::{Deserialize, Serialize};
use crate::actions::Action;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct ControllableBehaviour {
	pub speed: f32,
	pub run_speed: f32,
	pub jumping: f32,
	pub velocity: PhysicsVector,
	pub acceleration: PhysicsVector,
}

impl ControllableBehaviour {
    pub fn new(speed: f32, run_speed: f32) -> Self {
        Self {
            speed,
            run_speed,
			jumping: 0.0,
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

        let speed = if actions.contains(Action::SPRINT) {
            self.run_speed
        } else {
            self.speed
        };

		let mut in_air = true;

		for i in 0..params.collisions.len() {
			let (_, collision, _) = params.collisions[i];

			if collision.w > collision.h && collision.top() > bounds.center().y {
				in_air = false;

				break;
			}
		}

		if !in_air && actions.contains(Action::JUMP) && self.jumping == 0.0 {
			self.jumping = 0.3;	// 0.3 sec cooldown for jumping; this is just a dirty fix for multi-jumps
			impulse += PhysicsVector { x: 0.0, y: -self.run_speed };
		}

		self.jumping = f32::max(0.0, self.jumping - sec);

        // if actions.contains(Action::MoveUp) {
        //     force -= PhysicsVector { x: 0.0, y: 1.0 };
        // } else if actions.contains(Action::MoveDown) {
        //     force += PhysicsVector { x: 0.0, y: 1.0 };
        // }

        if actions.contains(Action::MOVE_LEFT) {
			force -= PhysicsVector { x: 1.0, y: 0.0 };
        } else if actions.contains(Action::MOVE_RIGHT) {
			force += PhysicsVector { x: 1.0, y: 0.0 };
        }

		if force.len() != 0.0 {
			force = force.normal() * speed;
		}

        BehaviourResult {
            bounds: None,
            collisions: None,
			force: Some(force),
			impulse: Some(impulse),
        }
    }
}
