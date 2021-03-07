use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::stuff::Stuff;
use crate::serialize_components;

#[derive(Clone, Serialize, Deserialize)]
pub struct Position(pub Point);

#[derive(Clone, Serialize, Deserialize)]
pub struct ContainsStuff {
    pub stuff: Stuff,
    pub amount: u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bomb;

#[derive(Clone, Serialize, Deserialize)]
pub struct Inventory(pub Vec<Entity>);

serialize_components!(
    Position,
    ContainsStuff,
    Bomb,
    Inventory
);
