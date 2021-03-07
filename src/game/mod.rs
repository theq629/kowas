pub mod terrain;
pub mod graphics;
pub mod components;
pub mod actions;
mod systems;
mod state;

use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;
pub use state::GameState;
use crate::tilemap::TileMap;
use terrain::Terrain;
use actions::Action;

fn make_terrain(dim: Point, rng: &mut RandomNumberGenerator) -> TileMap<Terrain> {
    let mut terrain = TileMap::new(dim, |_| Terrain::Empty);

    let mut y = 0;
    loop {
        y += rng.range(2, 8);
        if y >= dim.y {
            break;
        }
        for x in 0..dim.x {
            if rng.range(0, 10) < 7 {
                terrain[Point::new(x, y)] = Terrain::Floor;
            }
        }
    }
    terrain
}

pub fn new_game() -> GameState {
    let mut world = World::new();

    let dim = Point::new(64, 128);
    let mut rng = RandomNumberGenerator::new();
    let terrain = make_terrain(dim, &mut rng);

    let player = world.spawn((
        components::Position(Point::new(dim.x / 2, 1)),
        components::Renderable(graphics::Graphic::Player)
    ));

    GameState {
        world,
        terrain,
        player
    }
}

pub fn tick(state: &mut GameState, player_action: Action) {
    match player_action {
        Action::DoNothing => {},
        Action::MoveLeft => {
            let mut query = state.world.query_one::<(&mut components::Position,)>(state.player).unwrap();
            let (pos,) = query.get().unwrap();
            pos.0.x -= 1;
        },
        Action::MoveRight => {
            let mut query = state.world.query_one::<(&mut components::Position,)>(state.player).unwrap();
            let (pos,) = query.get().unwrap();
            pos.0.x += 1;
        }
    }
    systems::gravity(state);
}
