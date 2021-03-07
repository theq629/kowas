use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;
use crate::serialize_components;

#[derive(Clone, Serialize, Deserialize)]
pub struct Position(pub Point);

serialize_components!(Position);
