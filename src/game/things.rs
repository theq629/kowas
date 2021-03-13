use bracket_geometry::prelude::Point;
use hecs::{Entity, World};
use crate::game::graphics::Graphic;
use crate::game::components;

pub fn player(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Player),
        components::Blocks,
        components::Health::new(50),
        components::Inventory(Vec::new()),
        components::Power(6),
        components::Energy { value: 50 },
        components::IsPlayer
    ))
}

pub fn goblin(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Goblin),
        components::Blocks,
        components::Health::new(1),
        components::ProvidesEnergy(1),
        components::Power(1),
        components::Speed(1),
        components::IsAi
    ))
}

pub fn orc(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Orc),
        components::Blocks,
        components::Health::new(5),
        components::ProvidesEnergy(5),
        components::Power(5),
        components::Speed(3),
        components::IsAi
    ))
}

pub fn orc_lord(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::OrcLord),
        components::Blocks,
        components::Health::new(10),
        components::ProvidesEnergy(1000),
        components::Power(10),
        components::Speed(1),
        components::IsAi,
        components::IsPlayerGoal
    ))
}
