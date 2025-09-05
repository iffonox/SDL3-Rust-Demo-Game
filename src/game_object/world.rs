use crate::serialization::Action;
use crate::serialization::behaviour::BehaviourType;
use crate::serialization::behaviour::BehaviourType::Collision;
use crate::serialization::level::LevelData;
use crate::game_object::behaviour::Behaviour;
use crate::game_object::behaviour::collision::CollisionBehaviour;
use crate::game_object::behaviour::controllable::ControllableBehaviour;
use crate::game_object::behaviour::dvd::DvdBehaviour;
use crate::game_object::{Bounds, Drawable, GameObject, PhysicsBody, PhysicsVector};
use sdl3::pixels::Color;
use sdl3::render::FRect;

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
    world_physics: PhysicsBody,
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
            world_physics: PhysicsBody::default(),
        }
    }

    pub fn load_level(&mut self, level_data: &LevelData) {
        if let Some(bounds) = level_data.bounds {
            self.bounds = bounds.into();
        }

        let player_data = &level_data.player;

        let mut player = GameObject::new(1);
        player.bounds = FRect {
            x: level_data.start.x,
            y: level_data.start.y,
            w: player_data.size.w,
            h: player_data.size.h,
        };
        player.drawable = Some(Drawable {
            z: 100,
            color: Color::MAGENTA,
            texture: Some(player_data.texture_id),
            tint_texture: true,
        });
        player.behaviours.push(Box::new(ControllableBehaviour::new(
            self.bounds,
            50.0,
            200.0,
        )));
        player
            .behaviours
            .push(Box::new(CollisionBehaviour::new(player.bounds)));

        self.add_game_object(player);

        let game_objects = &level_data.objects;

        for i in 0..game_objects.len() {
            let data = &game_objects[i];
            let mut game_object = GameObject::new(i as i32 + 10);

            game_object.bounds = data.bounds.into();
            game_object.drawable = Some(Drawable {
                z: 50,
                color: data
                    .color
                    .map(|a| a.into())
                    .unwrap_or_else(|| Color::MAGENTA),
                texture: data.texture_id,
                tint_texture: data.tint_texture.unwrap_or_default(),
            });

            let behaviours = &data.behaviours;

            for j in 0..behaviours.len() {
                let behaviour_data = &behaviours[j];

                if let Some(behaviour) =
                    Self::build_behaviour(behaviour_data, game_object.bounds, self.bounds)
                {
                    game_object.behaviours.push(behaviour);
                }
            }

            self.add_game_object(game_object);
        }
    }

    fn build_behaviour(
        behaviour_data: &BehaviourType,
        object_bounds: FRect,
        world_bounds: FRect,
    ) -> Option<Box<dyn Behaviour>> {
        let behaviour: Option<Box<dyn Behaviour>> = match behaviour_data {
            BehaviourType::Dvd { bounds, speed, .. } => {
                let bounds: FRect = bounds.map(|b| b.into()).unwrap_or_else(|| world_bounds);
                let speed = PhysicsVector::from(*speed);

                Some(Box::new(DvdBehaviour::new(bounds, speed)))
            }
            Collision { bounds, .. } => {
                let bounds: FRect = bounds.map(|b| b.into()).unwrap_or_else(|| object_bounds);

                Some(Box::new(CollisionBehaviour::new(bounds)))
            }
            _ => None,
        };
        behaviour
    }

    fn add_game_object(&mut self, object: GameObject) {
        self.game_objects.push(object);
        self.game_objects
            .sort_by_key(|b| b.drawable.as_ref().map(|d| d.z).unwrap_or_default());
    }

    pub fn tick(&mut self, delta_t: f64, actions: Action) {
        let rects: Vec<(i32, FRect)> = self.game_objects.iter().map(|o| (o.id, o.bounds)).collect();

        for i in 0..self.game_objects.len() {
            let game_object = &mut self.game_objects[i];

            game_object.tick(delta_t, actions, &rects);
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
