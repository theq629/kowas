use bracket_geometry::prelude::Point;
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::stuff::Stuff;
use crate::game::directions::Direction;
use crate::game::components::{Position, Inventory};
use super::change::{ChangeResult, ChangeOk};

pub fn get(entity: Entity, dir: Direction, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(entity)?.0.clone();
    let target_pos = pos + dir.to_point();

    let targets: Vec<(Entity, Point)> = state.world.query::<(&mut Position,)>()
        .iter()
        .filter(|(_, (p,))| p.0 == target_pos)
        .map(|(e, (p,))| (e, p.0))
        .collect();

    {
        let mut inv = state.world.get_mut::<Inventory>(entity)?;
        for (target, _) in targets.iter() {
            inv.0.push(*target);
        }
    }

    for (target, target_pos) in targets.iter() {
        // TODO: how to fill in?
        state.stuff[*target_pos] = Stuff::Air;
        state.world.remove_one::<Position>(*target)?;
    }

    Ok(ChangeOk)
}
