use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;
use crate::game::graphics::Graphic;
use crate::game::max_estimate::MaxEstimate;
use crate::serialize_components;

#[derive(Clone, Serialize, Deserialize)]
pub struct Position(pub Point);

#[derive(Clone, Serialize, Deserialize)]
pub struct Renderable(pub Graphic);

#[derive(Clone, Serialize, Deserialize)]
pub struct Blocks;

#[derive(Clone, Serialize, Deserialize)]
pub struct Health {
    pub value: i32
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            value: max
        }
    }

    pub fn change(&mut self, delta: i32) {
        self.value += delta;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaxHealthEstimate {
    pub estimate: MaxEstimate
}

impl MaxHealthEstimate {
    pub fn new() -> Self {
        Self {
            estimate: MaxEstimate::new()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Flying {
    pub velocity: Point
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Power(pub i32);

#[derive(Clone, Serialize, Deserialize)]
pub struct Energy {
    pub value: i32
}

impl Energy {
    pub fn new(max: i32) -> Self {
        Self {
            value: max
        }
    }

    pub fn change(&mut self, delta: i32) {
        self.value += delta;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaxEnergyEstimate {
    pub estimate: MaxEstimate
}

impl MaxEnergyEstimate {
    pub fn new() -> Self {
        Self {
            estimate: MaxEstimate::new()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProvidesEnergy(pub i32);

#[derive(Clone, Serialize, Deserialize)]
pub struct IsPlayer;

#[derive(Clone, Serialize, Deserialize)]
pub struct IsPlayerGoal;

#[derive(Clone, Serialize, Deserialize)]
pub struct IsAi;

#[derive(Clone, Serialize, Deserialize)]
pub struct Speed(pub u32);

serialize_components!(
    Position,
    Renderable,
    Blocks,
    Health,
    Flying,
    Power,
    Energy,
    ProvidesEnergy,
    IsPlayer,
    IsPlayerGoal,
    IsAi,
    Speed
);
