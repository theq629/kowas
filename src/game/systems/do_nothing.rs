use hecs::Entity;
use super::change::{ChangeResult, ChangeOk};

pub fn do_nothing(_actor: Entity) -> ChangeResult {
    Ok(ChangeOk)
}
