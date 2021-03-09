use enum_map::Enum;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Liquid {
    Blood,
    Gore
}
