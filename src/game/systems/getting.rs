use hecs::Entity;
use crate::game::state::GameState;
use crate::game::components::{Position, Inventory};
use super::change::{ChangeResult, ChangeOk};

pub fn get(entity: Entity, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(entity)?.0.clone();

    let targets: Vec<_> = state.world.query::<(&mut Position,)>()
        .iter()
        .filter(|(e, (p,))| *e != entity && p.0 == pos)
        .map(|(e, _)| e)
        .collect();

    {
        let mut inv = state.world.get_mut::<Inventory>(entity)?;
        for target in targets.iter() {
            inv.0.push(*target);
        }
    }

    for target in targets.iter() {
        state.world.remove_one::<Position>(*target)?;
    }

    Ok(ChangeOk)
}
