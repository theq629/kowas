use hecs::Entity;
use crate::game::state::GameState;
use crate::game::components::{Position, Inventory};
use super::change::{ChangeResult, ChangeOk};

pub fn get(getter: Entity, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(getter)?.0.clone();

    let targets: Vec<_> = state.world.query::<(&mut Position,)>()
        .iter()
        .filter(|(e, (p,))| *e != getter && p.0 == pos)
        .map(|(e, _)| e)
        .collect();

    {
        let mut inv = state.world.get_mut::<Inventory>(getter)?;
        for target in targets.iter() {
            inv.0.push(*target);
        }
    }

    for target in targets.iter() {
        state.world.remove_one::<Position>(*target)?;
    }

    Ok(ChangeOk)
}

pub fn drop(dropper: Entity, droppee: Entity, state: &mut GameState) -> ChangeResult {
    let pos = state.world.get::<Position>(dropper)?.0.clone();

    state.world.insert_one(droppee, Position(pos))?;

    {
        let mut inv = state.world.get_mut::<Inventory>(dropper)?;
        inv.0.retain(|e| *e != droppee)
    }

    Ok(ChangeOk)
}
