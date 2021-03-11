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

fn gen_rect(start: Point, end: Point, new_ter: Terrain, terrain: &mut TileMap<Terrain>) {
    for x in start.x..end.x {
        terrain[Point::new(x, start.y)] = new_ter;
        terrain[Point::new(x, end.y - 1)] = new_ter;
    }
    for y in (start.y + 1)..(end.y - 1) {
        terrain[Point::new(start.x, y)] = new_ter;
        terrain[Point::new(start.x - 1, y)] = new_ter;
    }
}

fn gen_rect_gapped(start: Point, end: Point, new_ter: Terrain, terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator) {
    let change_prob = 20;

    for y in vec![start.y, end.y - 1] {
        let mut drawing = false;
        for x in start.x..end.x {
            if rng.range(0, 100) < change_prob {
                drawing = !drawing;
            }
            if drawing {
                terrain[Point::new(x, y)] = new_ter;
            }
        }
    }
    for x in vec![start.x, end.x - 1] {
        let mut drawing = false;
        for y in start.y..end.y {
            if rng.range(0, 100) < change_prob {
                drawing = !drawing;
            }
            if drawing {
                terrain[Point::new(x, y)] = new_ter;
            }
        }
    }
}

fn gen_rect_filled(start: Point, end: Point, new_ter: Terrain, terrain: &mut TileMap<Terrain>) {
    for x in start.x..end.x {
        for y in start.y..end.y {
            terrain[Point::new(x, y)] = new_ter;
        }
    }
}

fn gen_terrain(terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator) {
    let dim = terrain.dim.clone();

    gen_rect(Point::zero(), dim, Terrain::BoundaryWall, terrain);
    let offset = Point::new(10, 10);
    gen_rect_gapped(offset, dim - offset, Terrain::Wall, terrain, rng);
    let offset = Point::new(20, 20);
    gen_rect_gapped(offset, dim - offset, Terrain::Wall, terrain, rng);

    for _ in 0..4 {
        let pos = Point::new(
            rng.range(10, dim.x - 10),
            rng.range(10, dim.y - 10)
        );
        gen_rect_filled(pos - Point::new(3, 3), pos + Point::new(3, 3), Terrain::Wall, terrain);
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
        gen_terrain(&mut terrain, rng);
    }

    let mut world = World::new();
    let liquids = TileMap::new(dim, |_| None);

    let player = things::player(Point::new(dim.x / 2, dim.y / 2), &mut world);
    for _ in 0..300 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        if !terrain[pos].is_solid() {
            things::goblin(pos, &mut world);
        }
    }
    for _ in 0..100 {
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
