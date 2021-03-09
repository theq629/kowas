use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::World;
use crate::tilemap::TileMap;
use crate::game::terrain::Terrain;
use crate::game::state::GameState;
use crate::game::things;

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> GameState {
    let terrain = TileMap::new(dim, |_| Terrain::Floor);
    let mut state = GameState {
        world: World::new(),
        terrain: terrain,
        player: None
    };

    let player = things::player(Point::new(dim.x / 2, dim.y / 2), &mut state);
    for _ in 0..50 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        things::goblin(pos, &mut state);
    }
    for _ in 0..10 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        things::orc(pos, &mut state);
    }
    state.player = Some(player);

    state
}
