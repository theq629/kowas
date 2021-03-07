use std::cmp::min;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use crate::tilemap::TileMap;
use crate::game::terrain::Terrain;

#[derive(Clone)]
struct Gap {
    start_x: i32,
    width: i32
}

struct Platform {
    start_y: i32,
    thickness: i32,
    gaps: Vec<Gap>
}

fn choose_platforms(dim: &Point, rng: &mut RandomNumberGenerator) -> Vec<Platform> {
    let mut platforms = Vec::new();

    let mut y = 0;
    loop {
        y += rng.range(2, 8);
        if y >= dim.y {
            break;
        }
        let thickness = min(dim.y - y, rng.range(1, 4));
        if thickness > 0 {
            platforms.push(Platform {
                start_y: y,
                thickness: thickness,
                gaps: Vec::new()
            });
        }
        y += thickness;
    }

    platforms
}

fn add_gaps(num_gaps: usize, dim: &Point, gaps: &mut Vec<Gap>, rng: &mut RandomNumberGenerator) {
    while gaps.len() < num_gaps {
        let x = rng.range(0, dim.x);
        let gap = Gap {
            start_x: x,
            width: min(dim.x - x, rng.range(1, 4))
        };
        gaps.push(gap)
    }
}

fn choose_gaps(platforms: &mut Vec<Platform>, dim: &Point, rng: &mut RandomNumberGenerator) {
    let min_gaps = dim.x / 20;
    let max_gaps = min_gaps * 2;
    let mut keep_gaps = Vec::<Gap>::new();
    for platform in platforms.iter_mut() {
        let num_gaps = rng.range(min_gaps, max_gaps);
        platform.gaps.append(&mut keep_gaps);
        add_gaps(num_gaps as usize, &dim, &mut platform.gaps, rng);
        for _ in 0..rng.range(0, (platform.gaps.len() as f32 * 0.6) as usize) {
            let i = rng.range(0, platform.gaps.len());
            keep_gaps.push(platform.gaps[i].clone());
        }
    }
}

fn draw_platforms(dim: &Point, platforms: &mut Vec<Platform>) -> TileMap<Terrain> {
    let mut terrain = TileMap::new(*dim, |_| Terrain::Floor);

    for platform in platforms.iter_mut() {
        platform.gaps.push(Gap {
            start_x: dim.x,
            width: 0
        });
        platform.gaps.sort_by_key(|g| g.start_x);
        let mut start_x = 0;
        for gap in platform.gaps.iter() {
            for x in start_x..gap.start_x {
                for y in platform.start_y..(platform.start_y + platform.thickness) {
                    terrain[Point::new(x, y)] = Terrain::Wall;
                }
            }
            start_x = gap.start_x + gap.width;
        }
    }

    terrain
}

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> TileMap<Terrain> {
    let mut platforms = choose_platforms(&dim, rng);
    choose_gaps(&mut platforms, &dim, rng);
    draw_platforms(&dim, &mut platforms)
}
