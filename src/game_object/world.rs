use std::collections::HashSet;
use crate::game_object::behaviour::collision::CollisionBehaviour;
use crate::game_object::behaviour::controllable::ControllableBehaviour;
use crate::game_object::behaviour::physics::PhysicsBehaviour;
use crate::game_object::behaviour::{BehaviourType};
use crate::game_object::{BoundInfo, Bounds, DrawLayer, Drawable, GameObject, PhysicsVector};
use crate::serialization::level::LevelData;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use crate::actions::Action;

#[derive(PartialEq, Eq, Debug, Default)]
pub enum BorderType {
    #[default]
    Solid,
    Fatal,
}

#[derive(PartialEq, Eq, Debug, Default)]
pub struct Borders {
    top: BorderType,
    bottom: BorderType,
    left: BorderType,
    right: BorderType,
}

pub struct World {
    bounds: FRect,
    scale: f32,
    borders: Borders,
    game_objects: Vec<GameObject>,
}

impl World {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            bounds: FRect {
                x: f32::default(),
                y: f32::default(),
                w,
                h,
            },
            scale: 1.0,
            borders: Borders::default(),
            game_objects: Vec::new(),
        }
    }

    pub fn load_level(&mut self, level_data: &LevelData) {
		let game_objects = &level_data.objects;
		self.game_objects = game_objects.clone();

		self.bounds = level_data.bounds;

		let player_data = &level_data.player;
        let mut player = GameObject::new(-1);
        player.bounds = FRect {
            x: level_data.start.x,
            y: level_data.start.y,
            w: player_data.size.w,
            h: player_data.size.h,
        };
        player.drawable = Some(Drawable {
            z: DrawLayer::Foreground(100),
            color: Some(Color::MAGENTA),
            texture_id: Some(player_data.texture_id),
            tint_texture: true,
        });
        player
            .behaviours
            .push(BehaviourType::Collision(CollisionBehaviour::new()));
        player
            .behaviours
            .push(BehaviourType::Controllable(ControllableBehaviour::new(
                5.0,
                15.0,
            )));
        player.behaviours.push(BehaviourType::Physics(PhysicsBehaviour::new(
            self.bounds,
            PhysicsVector::default(),
            2.0,
        )));

        self.add_game_object(player);
    }

    fn add_game_object(&mut self, object: GameObject) {
        self.game_objects.push(object);
        self.game_objects
            .sort_by_key(|b| b.drawable.as_ref().map(|d| d.z).unwrap());
    }

    pub fn tick(&mut self, delta_t: f64, actions: &HashSet<Action>) {
        let rects: Vec<BoundInfo> = self.game_objects.iter().map(|o| (o.id, o.bounds, o.mask)).collect();

        for i in 0..self.game_objects.len() {
            let game_object = &mut self.game_objects[i];

            game_object.tick(delta_t, self.bounds, actions, &rects);
        }
    }

    pub fn get_drawables(&self) -> Vec<(FRect, &Drawable)> {
        let mut vec = Vec::new();

        for i in 0..self.game_objects.len() {
            let game_object = &self.game_objects[i];
            let id = game_object.id;

            if let Some(drawable) = game_object.drawable.as_ref() {
                if !self.is_visible(game_object.bounds) {
                    let a = format!("not in view {id}");
                    dbg!(a);
                    continue;
                }

                vec.push((game_object.bounds, drawable));
            }
        }

        vec
    }

    fn is_visible(&self, bounds: FRect) -> bool {
        bounds.intersects(self.bounds)
    }
}
