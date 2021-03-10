use bracket_geometry::prelude::Point;
use crate::game::state::GameState;
use crate::game::graphics::Graphic;
use crate::game::components::{Position, Renderable};

struct Particle {
    life: u32
}

pub fn make_particle(pos: Point, graphic: Graphic, state: &mut GameState) {
    state.particles_world.spawn((
        Particle { life: 8 },
        Renderable(graphic),
        Position(pos)
    ));
}

pub fn tick_particles(state: &mut GameState) {
    for (_, mut particle) in state.particles_world.query::<&mut Particle>().iter() {
        particle.life -= 1;
    }
    let done: Vec<_> = state.particles_world.query::<&mut Particle>()
        .iter()
        .filter(|(_, p)| p.life <= 0)
        .map(|(e, _)| e)
        .collect();
    for entity in done.iter() {
        let _ = state.particles_world.despawn(*entity);
    }
}
