use crate::serialization::AssetBounds;
use crate::game_object::PhysicsVector;
use crate::util::random;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type")]
pub enum BehaviourSpeed {
    Fixed(PhysicsVector),
    Random {
        min: PhysicsVector,
        max: PhysicsVector,
    },
}

impl From<BehaviourSpeed> for PhysicsVector {
    fn from(value: BehaviourSpeed) -> Self {
        match value {
            BehaviourSpeed::Fixed(v) => v,
            BehaviourSpeed::Random { min, max } => Self {
                x: random(min.x, max.x),
                y: random(min.y, max.y),
            },
        }
    }
}

impl From<PhysicsVector> for BehaviourSpeed {
    fn from(value: PhysicsVector) -> Self {
        BehaviourSpeed::Fixed(value)
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BehaviourType {
    Dvd {
        bounds: Option<AssetBounds>,
        speed: BehaviourSpeed,
    },
    Controllable {
        bounds: Option<AssetBounds>,
        speed: f32,
        run_speed: f32,
    },
    Collision {
        bounds: Option<AssetBounds>,
    },
}
