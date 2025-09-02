use crate::game_object::{Bounds, Drawable, GameObject, PhysicsBody};
use sdl3::render::FRect;
use crate::game::Action;

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
    rect: FRect,
    scale: f32,
    borders: Borders,
    game_objects: Vec<GameObject>,
    world_physics: PhysicsBody,
}

impl World {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            rect: FRect {
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

    pub fn add_game_object(&mut self, object: GameObject) {
        self.game_objects.push(object);
        self.game_objects
            .sort_by_key(|b| b.drawable.as_ref().map(|d| d.z).unwrap_or_default());
    }

    pub fn tick(&mut self, delta_t: u64, actions: Action) {
		let rects: Vec<(i32, FRect)> = self.game_objects.iter().map(|o| (o.id, o.bounds)).collect();
		
        for i in 0..self.game_objects.len() {
            let game_object = &mut self.game_objects[i];

            let result = game_object.tick(delta_t, actions, &rects);

			if let Some(bounds) = result.bounds {
				game_object.bounds = bounds;
			}
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
        bounds.intersects(self.rect)
    }
}
