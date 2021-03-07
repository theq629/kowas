pub mod stuff;
pub mod graphics;
pub mod components;
pub mod directions;
pub mod actions;
mod systems;
mod state;

use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::{World, Entity};
pub use state::GameState;
use crate::tilemap::TileMap;
use stuff::Stuff;
use actions::Action;
use directions::Direction;

fn make_terrain(dim: Point, rng: &mut RandomNumberGenerator) -> TileMap<Stuff> {
    let mut terrain = TileMap::new(dim, |_| Stuff::Air);

    let mut y = 0;
    loop {
        y += rng.range(2, 8);
        if y >= dim.y {
            break;
        }
        for x in 0..dim.x {
            if rng.range(0, 10) < 7 {
                terrain[Point::new(x, y)] = Stuff::Floor;
            }
        }
    }
    terrain
}

pub fn make_player(pos: Point, state: &mut GameState) -> Entity {
    state.stuff[pos] = Stuff::Body;
    state.world.spawn((
        components::Position(pos),
    ))
}

pub fn new_game() -> GameState {
    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    let stuff = make_terrain(dim, &mut rng);

    let mut state = GameState {
        world: World::new(),
        stuff: stuff,
        player: None
    };

    let player = make_player(Point::new(dim.x / 2, 1), &mut state);
    state.player = Some(player);

    state
}

pub fn tick(state: &mut GameState, player_action: Action) {
    match player_action {
        Action::DoNothing => {},
        Action::MoveLeft => {
            if let Some(player) = state.player {
                systems::move_entity(player, Direction::Left, state);
            }
        },
        Action::MoveRight => {
            if let Some(player) = state.player {
                systems::move_entity(player, Direction::Right, state);
            }
        }
    }
    systems::apply_gravity(state);
}
