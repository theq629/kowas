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

fn gen_terrain(terrain: &mut TileMap<Terrain>) {
    let dim = terrain.dim.clone();
    for x in 0..dim.x {
        terrain[Point::new(x, 0)] = Terrain::Wall;
        terrain[Point::new(x, dim.y - 1)] = Terrain::Wall;
    }
    for y in 0..dim.y {
        terrain[Point::new(0, y)] = Terrain::Wall;
        terrain[Point::new(dim.x - 1, y)] = Terrain::Wall;
    }

    let bld_half_dim_x = dim.x / 16;
    let bld_half_dim_y = dim.y / 16;
    for x in (dim.x / 2 - bld_half_dim_x)..(dim.x / 2 + bld_half_dim_x) {
        for y in (dim.y / 2 - bld_half_dim_y)..(dim.y / 2 + bld_half_dim_y) {
            terrain[Point::new(x, y)] = Terrain::FloorUnderRoof;
        }
    }
    for x in (dim.x / 2 - bld_half_dim_x)..(dim.x / 2 + bld_half_dim_x) {
        terrain[Point::new(x, dim.y / 2 - bld_half_dim_y)] = Terrain::Wall;
        terrain[Point::new(x, dim.y / 2 + bld_half_dim_y)] = Terrain::Wall;
    }
}

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> GeneratedWorld {
    let mut terrain = TileMap::new(dim, |_| Terrain::Floor);
    {
        gen_terrain(&mut terrain);
    }

    let mut world = World::new();
    let liquids = TileMap::new(dim, |_| None);

    let player = things::player(Point::new(dim.x / 2, dim.y / 2), &mut world);
    for _ in 0..50 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        if !terrain[pos].is_solid() {
            things::goblin(pos, &mut world);
        }
    }
    for _ in 0..10 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        if !terrain[pos].is_solid() {
            things::orc(pos, &mut world);
        }
    }

    let mut state = GeneratedWorld {
        world: world,
        terrain: terrain,
        liquids: liquids,
        player: None
    };
    state.player = Some(player);

    state
}
