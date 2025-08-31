pub mod world;

extern crate sdl3;

use crate::math::bounds::Bounds;
use crate::math::vector2::Vector2;
use sdl3::pixels::Color;
use sdl3::render::{FRect};
use crate::game::Action;

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
    pub fn apply_force(&mut self, game_object: &GameObject, force: &PhysicsVector) {}

    pub fn apply(&self, game_object: &mut GameObject) {}
}

pub struct ActionHandler {}

#[derive(Clone, Copy)]
pub struct BehaviourParameter {
    bounds: FRect,
    actions: Action,
}

pub struct BehaviourResult {
    bounds: Option<FRect>,
}

pub trait Behaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: u64) -> BehaviourResult;
}

pub struct DvdBehaviour {
    bounds: FRect,
    speed: PhysicsVector,
}

impl DvdBehaviour {
    pub fn new(bounds: FRect, speed: PhysicsVector) -> Self {
        Self {
            bounds,
            speed,
        }
    }
}

impl Behaviour for DvdBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: u64) -> BehaviourResult {
        let sec = delta_t as f32 / 1000.0;
        let center = params.bounds.center();
        let mut position = PhysicsVector::from(center);

        position = position + self.speed * sec;

        if position.x < self.bounds.left() {
            self.speed.x = -self.speed.x;
            position.x = self.bounds.left();
        } else if position.x > self.bounds.right() {
            self.speed.x = -self.speed.x;
            position.x = self.bounds.right();
        }

        if position.y < self.bounds.top() {
            self.speed.y = -self.speed.y;
            position.y = self.bounds.top();
        } else if position.y > self.bounds.bottom() {
            self.speed.y = -self.speed.y;
            position.y = self.bounds.bottom();
        }

        let mut bounds = params.bounds;

        bounds.set_center(position);

        BehaviourResult {
            bounds: Some(bounds),
        }
    }
}

pub struct ControllableBehaviour {
    bounds: FRect,
    speed: f32,
    run_speed: f32,
}

impl ControllableBehaviour {
    pub fn new(bounds: FRect, speed: f32, run_speed: f32) -> Self {
        Self {
            bounds,
            speed,
            run_speed
        }
    }
}

impl Behaviour for ControllableBehaviour {
    fn tick(&mut self, params: BehaviourParameter, delta_t: u64) -> BehaviourResult {
        let sec = delta_t as f32 / 1000.0;
        let actions = params.actions;
        let center = params.bounds.center();
        let mut position = PhysicsVector::from(center);
        let speed = if actions.contains(Action::Sprint) { self.run_speed } else { self.speed };

        if actions.contains(Action::MoveUp) {
            position.y -= speed * sec;
        } else if actions.contains(Action::MoveDown) {
            position.y += speed * sec;
        }

        if actions.contains(Action::MoveLeft) {
            position.x -= speed * sec;
        } else if actions.contains(Action::MoveRight) {
            position.x += speed * sec;
        }

        position.x = position.x.clamp(self.bounds.left(), self.bounds.right());
        position.y = position.y.clamp(self.bounds.top(), self.bounds.bottom());

        let mut bounds = params.bounds;

        bounds.set_center(position);

        BehaviourResult {
            bounds: Some(bounds),
        }
    }
}

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
            drawable: Some(Drawable {
                z: i32::default(),
                color: Color::BLACK,
                texture: usize::default(),
            }),
            physics_body: None,
            action_handler: None,
            behaviours: Vec::new(),
        }
    }

    pub fn tick(&mut self, delta_t: u64, actions: Action) {
        let behaviours = &mut self.behaviours;
        let mut bounds = self.bounds;

        for i in 0..behaviours.len() {
            let behaviour = behaviours[i].as_mut();

            let result = behaviour.tick(BehaviourParameter { bounds, actions }, delta_t);

            if let Some(b) = result.bounds {
                bounds = b
            }
        }

        self.bounds = bounds;
    }
}
