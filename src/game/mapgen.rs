use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use hecs::{World, Entity};
use crate::tilemap::TileMap;
use crate::game::terrain::Terrain;
use crate::game::liquids::Liquid;
use crate::game::things;

pub struct GeneratedWorld {
    pub world: World,
    pub terrain: TileMap<Terrain>,
    pub liquids: TileMap<Option<Liquid>>,
    pub player: Option<Entity>
}

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> GeneratedWorld {
    let terrain = TileMap::new(dim, |_| Terrain::Floor);
    let liquids = TileMap::new(dim, |_| None);
    let mut state = GeneratedWorld {
        world: World::new(),
        terrain: terrain,
        liquids: liquids,
        player: None
    };

    let player = things::player(Point::new(dim.x / 2, dim.y / 2), &mut state.world);
    for _ in 0..50 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        things::goblin(pos, &mut state.world);
    }
    for _ in 0..10 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        things::orc(pos, &mut state.world);
    }
    state.player = Some(player);

    state
}
