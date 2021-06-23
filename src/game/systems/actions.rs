use log::debug;
use hecs::Entity;
use crate::game::state::GameState;
use crate::game::actions::Action;
use crate::game::components::Energy;
use super::change::{ChangeResult, ChangeOk, ChangeErr};

fn dispatch_action(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    match action {
        Action::DoNothing => {
            debug!("do nothing");
            super::do_nothing(actor)
        },
        Action::Move(dir) => {
            debug!("move");
            super::move_entity(actor, dir, state)
        },
        Action::MeleeAttack(dir) => {
            debug!("melee attack");
            super::melee_attack_toward(actor, dir, state)
        },
        Action::Shove(dir) => {
            debug!("shove");
            super::body::shove_toward(actor, dir, state)
        },
        Action::ThrowOff => {
            debug!("throw off");
            super::body::throw_off(actor, state)
        },
        Action::Heal => {
            debug!("heal");
            super::body::heal(actor, state)
        },
        Action::SwordSlash(dir) => {
            debug!("sword slash");
            super::sword::slash_toward(actor, dir, state)
        },
        Action::SwordWhirl => {
            debug!("sword whirl");
            super::sword::whirl(actor, state)
        },
        Action::SwordFlurry(dir) => {
            debug!("sword flurry");
            super::sword::flurry_toward(actor, dir, state)
        },
        Action::GetALotOfEnergy => {
            debug!("get a lot of energy");
            super::get_a_lot_of_energy(actor, state)
        },
        Action::GetALotOfHealth => {
            debug!("get a lot of health");
            super::get_a_lot_of_health(actor, state)
        },
        Action::GainPower => {
            debug!("gain power");
            super::gain_power(actor, state)
        },
    }
}

fn run_action(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    let needs_energy = action.needs_energy();
    if needs_energy > 0 {
        let actor_energy = state.world.get::<Energy>(actor)?;
        if actor_energy.value < needs_energy {
            return Err(ChangeErr);
        }
    }
    dispatch_action(actor, action, state)?;
    if let Ok(mut actor_energy) = state.world.get_mut::<Energy>(actor) {
        actor_energy.change(-needs_energy);
    }
    Ok(ChangeOk)
}

pub fn act(actor: Entity, action: Action, state: &mut GameState) -> ChangeResult {
    debug!("acting");
    run_action(actor, action, state).and_then(|ok| {
        debug!("updating flying");
        super::update_flying(state);
        debug!("checking win or lose");
        super::check_win_lose(state);
        debug!("checking deaths");
        super::check_deaths(state);
        if let Some(player) = state.player {
            if !state.world.contains(player) {
                state.player = None;
            }
        }
        debug!("done updates");
        Ok(ok)
    })
}
