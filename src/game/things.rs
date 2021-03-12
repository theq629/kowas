use bracket_geometry::prelude::Point;
use hecs::{Entity, World};
use crate::game::graphics::Graphic;
use crate::game::components;

pub fn player(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Player),
        components::Blocks,
        components::Health::new(20),
        components::Inventory(Vec::new()),
        components::Power(1),
        components::Energy { value: 10 }
    ))
}

pub fn goblin(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Goblin),
        components::Blocks,
        components::Health::new(1),
        components::ProvidesEnergy(1)
    ))
}

pub fn orc(pos: Point, world: &mut World) -> Entity {
    world.spawn((
        components::Position(pos),
        components::Renderable(Graphic::Orc),
        components::Blocks,
        components::Health::new(5),
        components::ProvidesEnergy(5)
    ))
}
