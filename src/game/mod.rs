pub mod terrain;
pub mod graphics;
pub mod components;
mod state;

use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;
pub use state::GameState;
use crate::tilemap::TileMap;
use terrain::Terrain;

fn make_terrain(dim: Point, rng: &mut RandomNumberGenerator) -> TileMap<Terrain> {
    let mut terrain = TileMap::new(dim, |_| Terrain::Empty);
    for _ in 0..10 {
        let pos = Point::new(rng.range(0, dim.x), rng.range(0, dim.y));
        terrain[pos] = Terrain::Floor;
    }
    terrain
}

pub fn new_game() -> GameState {
    let mut world = World::new();

    let dim = Point::new(8, 16);
    let mut rng = RandomNumberGenerator::new();
    let terrain = make_terrain(dim, &mut rng);

    let player = world.spawn((
        components::Position(Point::new(4, 1)),
        components::Renderable(graphics::Graphic::Player)
    ));

    GameState {
        world,
        terrain,
        player
    }
}
