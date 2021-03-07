use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use crate::tilemap::TileMap;
use crate::game::stuff::Stuff;

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> TileMap<Stuff> {
    let mut stuff = TileMap::new(dim, |_| Stuff::Air);

    let mut y = 0;
    loop {
        y += rng.range(2, 8);
        if y >= dim.y {
            break;
        }
        for x in 0..dim.x {
            if rng.range(0, 10) < 7 {
                stuff[Point::new(x, y)] = Stuff::Floor;
            }
        }
    }

    stuff
}
