use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;
use crate::serialize_components;
use super::graphics::Graphic;

#[derive(Clone, Serialize, Deserialize)]
pub struct Position(pub Point);

#[derive(Clone, Serialize, Deserialize)]
pub struct Renderable(pub Graphic);

serialize_components!(Position, Renderable);
