use std::cmp::min;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use bracket_pathfinding::prelude::{SmallVec, BaseMap, a_star_search, DijkstraMap};
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

struct Room {
    start: Point,
    end: Point,
    centre: Point
}

impl Room {
    pub fn new(start: Point, end: Point) -> Self {
        let centre = (start + end) / 2;
        Room {
            start: start,
            end: end,
            centre: centre
        }
    }
}

struct TerrainPather<'a> {
    terrain: &'a TileMap<Terrain>,
    rubble_weight: f32
}

impl <'a> TerrainPather<'a> {
    pub fn new(terrain: &'a TileMap<Terrain>) -> Self {
        Self {
            terrain: terrain,
            rubble_weight: (terrain.dim.x + terrain.dim.y) as f32
        }
    }
}

impl<'a> BaseMap for TerrainPather<'a> {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let x = idx as i32 % self.terrain.dim.x;
        let y = idx as i32 / self.terrain.dim.x;
        let mut exits = SmallVec::new();

        if self.terrain[idx].is_solid() {
            return exits;
        }

        let weight =
            if self.terrain[idx] == Terrain::Rubble {
                self.rubble_weight
            } else {
                1.0
            };

        if y > 0 {
            let i = idx - self.terrain.dim.x as usize;
            if self.terrain[i] != Terrain::BoundaryWall {
                exits.push((i, weight));
            }
        }
        if x > 0 {
            let i = idx - 1;
            if self.terrain[i] != Terrain::BoundaryWall {
                exits.push((i, weight));
            }
        }
        if x < self.terrain.dim.x - 1 {
            let i = idx + 1;
            if self.terrain[i] != Terrain::BoundaryWall {
                exits.push((i, weight));
            }
        }
        if y < self.terrain.dim.y - 1 {
            let i = idx + self.terrain.dim.x as usize;
            if self.terrain[i] != Terrain::BoundaryWall {
                exits.push((i, weight));
            }
        }

        exits
    }
}

fn gen_rect(start: Point, end: Point, new_ter: Terrain, terrain: &mut TileMap<Terrain>) {
    for x in start.x..end.x {
        terrain[Point::new(x, start.y)] = new_ter;
        terrain[Point::new(x, end.y- 1)] = new_ter;
    }
    for y in (start.y + 1)..end.y {
        terrain[Point::new(start.x, y)] = new_ter;
        terrain[Point::new(start.x - 1, y)] = new_ter;
    }
}

fn gen_horiz_wall(start_x: i32, end_x: i32, y: i32, terrain: &mut TileMap<Terrain>) {
    for x in start_x..end_x {
        let pos = Point::new(x, y);
        if !terrain[pos].is_solid() {
            terrain[pos] = Terrain::Wall;
        }
    }
}

fn gen_vert_wall(x: i32, start_y: i32, end_y: i32, terrain: &mut TileMap<Terrain>) {
    for y in start_y..end_y {
        let pos = Point::new(x, y);
        if !terrain[pos].is_solid() {
            terrain[pos] = Terrain::Wall;
        }
    }
}

fn add_doors(start: Point, end: Point, margin: i32, prob: i32, terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator) {
    let choose = |prob: i32, rng: &mut RandomNumberGenerator| {
        if rng.range(0, 100) < prob {
            Terrain::Floor
        } else {
            Terrain::Rubble
        }
    };
    if end.y - start.y > margin * 2 {
        if start.x > 1 {
            let y = rng.range(start.y + margin, end.y - margin);
            terrain[Point::new(start.x - 1, y)] = choose(prob, rng);
        }
        if end.x < terrain.dim.x - 1 {
            let y = rng.range(start.y + margin, end.y - margin);
            terrain[Point::new(end.x + 1, y)] = choose(prob, rng);
        }
    }
    if end.x - start.x > margin * 2 {
        if start.y > 1 {
            let x = rng.range(start.x + margin, end.x - margin);
            terrain[Point::new(x, start.y - 1)] = choose(prob, rng);
        }
        if end.y < terrain.dim.y - 1 {
            let x = rng.range(start.x + margin, end.x - margin);
            terrain[Point::new(x, end.y + 1)] = choose(prob, rng);
        }
    }
}

fn subdivide(start: Point, end: Point, depth: i32, terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator, rooms: &mut Vec<Room>) {
    let min_size = 6;
    let margin = min_size / 2;
    let stop_prob = 70;
    let min_size_to_stop = 16;
    let door_margin = 1;
    let door_prob = 25;

    if depth > 16 {
        rooms.push(Room::new(start, end));
        return;
    }

    let over_min_x = end.x - start.x > min_size;
    let over_min_y = end.y - start.y > min_size;

    if end.x - start.x < min_size_to_stop && end.y - start.y < min_size_to_stop {
        if rng.range(0, 100) < stop_prob {
            rooms.push(Room::new(start, end));
            return;
        }
    }

    add_doors(start, end, door_margin, door_prob, terrain, rng);

    if over_min_y && (!over_min_x || rng.range(0, 100) < 50) {
        let y = rng.range(start.y + margin, end.y - margin);
        gen_horiz_wall(start.x, end.x + 1, y, terrain);
        subdivide(start, Point::new(end.x, y - 1), depth + 1, terrain, rng, rooms);
        subdivide(Point::new(start.x, y + 1), end, depth + 1, terrain, rng, rooms);
    } else if over_min_x {
        let x = rng.range(start.x + margin, end.x - margin);
        gen_vert_wall(x, start.y, end.y + 1, terrain);
        subdivide(start, Point::new(x - 1, end.y), depth + 1, terrain, rng, rooms);
        subdivide(Point::new(x + 1, start.y), end, depth + 1, terrain, rng, rooms);
    } else {
        rooms.push(Room::new(start, end));
    }
}

fn fill_room(room: &Room, terrain: &mut TileMap<Terrain>) {
    for x in (room.start.x - 1)..(min(terrain.dim.x, room.end.x + 2)) {
        for y in (room.start.y - 1)..(min(terrain.dim.y, room.end.y + 2)) {
            let pos = Point::new(x, y);
            if terrain[pos] != Terrain::BoundaryWall {
                terrain[pos] = Terrain::Wall;
            }
        }
    }
}

fn guarantee_path(start: Point, end: Point, terrain: &mut TileMap<Terrain>) {
    let pather = TerrainPather::new(terrain);
    let path = a_star_search(
        terrain.to_location(start),
        terrain.to_location(end),
        &pather
    );
    if path.success {
        for pos_i in path.steps {
            let pos = terrain.from_location(pos_i);
            terrain[pos] = Terrain::Floor;
        }
    }
}

fn reachablize_rooms(rooms: &Vec<Room>, start_room_i: usize, terrain: &mut TileMap<Terrain>) {
    let start_loc = terrain.to_location(rooms[start_room_i].centre);
    let starts = vec![start_loc];
    let max_steps = terrain.dim.x + terrain.dim.y;

    let mut to_clear = Vec::new();
    {
        let pather = TerrainPather::new(terrain);
        let dijkstra_map = DijkstraMap::new(terrain.dim.x, terrain.dim.y, &starts, &pather, max_steps as f32);
        for room in rooms.iter() {
            let centre_loc = terrain.to_location(room.centre);
            if terrain[centre_loc] == Terrain::Wall {
                continue;
            }
            if dijkstra_map.map[centre_loc] == std::f32::MAX {
                let mut loc = centre_loc;
                for _ in 0..max_steps {
                    if let Some(next_loc) = DijkstraMap::find_lowest_exit(&dijkstra_map, loc, &pather) {
                        loc = next_loc;
                    } else {
                        break;
                    }
                    to_clear.push(loc);
                }
            }
        }
    }

    for loc in to_clear {
        terrain[loc] = Terrain::Floor;
    }
}

fn fill_some_rooms(num: u32, rooms: &Vec<Room>, dont_fill: &Vec<usize>, terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator) {
    let mut to_fill = Vec::new();
    for _ in 0..num {
        to_fill.push(rng.range(0, rooms.len()));
    }

    for room_i in to_fill {
        if !dont_fill.contains(&room_i) {
            fill_room(&rooms[room_i], terrain);
        }
    }
}

fn remove_rubble(terrain: &mut TileMap<Terrain>) {
    let dim = terrain.dim.clone();
    for x in 0..dim.x {
        for y in 0..dim.y {
            let pos = Point::new(x, y);
            if terrain[pos] == Terrain::Rubble {
                terrain[pos] = Terrain::Wall;
            }
        }
    }
}

fn gen_terrain(terrain: &mut TileMap<Terrain>, rng: &mut RandomNumberGenerator) -> (Vec<Room>, usize, usize) {
    let dim = terrain.dim.clone();

    gen_rect(Point::zero(), dim, Terrain::BoundaryWall, terrain);

    let mut rooms = Vec::new();
    subdivide(Point::new(1, 1), dim - Point::new(1, 1), 0, terrain, rng, &mut rooms);

    let (start_room_i, _) = rooms.iter().enumerate()
        .max_by_key(|(_, r)| r.centre.y)
        .unwrap();
    let (end_room_i, _) = rooms.iter().enumerate()
        .min_by_key(|(_, r)| r.centre.y)
        .unwrap();

    let dont_fill = vec![start_room_i, end_room_i];
    fill_some_rooms(rooms.len() as u32 / 10, &rooms, &dont_fill, terrain, rng);
    guarantee_path(rooms[start_room_i].centre, rooms[end_room_i].centre, terrain);
    reachablize_rooms(&rooms, start_room_i, terrain);
    remove_rubble(terrain);

    (rooms, start_room_i, end_room_i)
}

pub fn gen_map(dim: Point, rng: &mut RandomNumberGenerator) -> GeneratedWorld {
    let (terrain, rooms, start_room, end_room) = {
            let mut terrain = TileMap::new(dim, |_| Terrain::Floor);
            let (rooms, start_room, end_room) = gen_terrain(&mut terrain, rng);
            (terrain, rooms, start_room, end_room)
        };

    let mut world = World::new();
    let liquids = TileMap::new(dim, |_| None);

    let player_pos = rooms[start_room].centre;
    let player = things::player(player_pos, &mut world);

    let goal_pos = rooms[end_room].centre;
    things::orc_lord(goal_pos, &mut world);

    for _ in 0..500 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        if !terrain[pos].is_solid() && pos != player_pos && pos != goal_pos {
            things::goblin(pos, &mut world);
        }
    }
    for _ in 0..200 {
        let pos = Point::new(
            rng.range(0, dim.x),
            rng.range(0, dim.y)
        );
        if !terrain[pos].is_solid() && pos != player_pos && pos != goal_pos {
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
