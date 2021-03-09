use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::graphics::Graphic;
use crate::serialize_components;

#[derive(Clone, Serialize, Deserialize)]
pub struct Position(pub Point);

#[derive(Clone, Serialize, Deserialize)]
pub struct Renderable(pub Graphic);

#[derive(Clone, Serialize, Deserialize)]
pub struct Blocks;

#[derive(Clone, Serialize, Deserialize)]
pub struct Health {
    pub value: i32,
    pub max: i32
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            value: max,
            max: max
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Inventory(pub Vec<Entity>);

serialize_components!(
    Position,
    Renderable,
    Blocks,
    Health,
    Inventory
);
