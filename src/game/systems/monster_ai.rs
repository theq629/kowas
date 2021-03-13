use log::debug;
use hecs::Entity;
use bracket_pathfinding::prelude::{SmallVec, BaseMap, DijkstraMap};
use crate::tilemap::TileMap;
use crate::log_err::result_error;
use crate::game::state::GameState;
use crate::game::directions::Direction;
use crate::game::terrain::Terrain;
use crate::game::actions::Action;
use crate::game::components::{Position, Speed, IsAi};
use super::change::{ChangeResult, ChangeOk};

struct TerrainPather<'a> {
    terrain: &'a TileMap<Terrain>
}

impl <'a> TerrainPather<'a> {
    pub fn new(terrain: &'a TileMap<Terrain>) -> Self {
        Self { terrain }
    }
}

impl<'a> BaseMap for TerrainPather<'a> {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let x = idx as i32 % self.terrain.dim.x;
        let y = idx as i32 / self.terrain.dim.x;
        let mut exits = SmallVec::new();

        let weight = 1.0;

        if y > 0 {
            let i = idx - self.terrain.dim.x as usize;
            if x > 0 && !self.terrain[i - 1].is_solid() {
                exits.push((i - 1, weight));
            }
            if !self.terrain[i].is_solid() {
                exits.push((i, weight));
            }
            if x < self.terrain.dim.x - 1 && !self.terrain[i + 1].is_solid() {
                exits.push((i + 1, weight));
            }
        }
        if x > 0 {
            let i = idx - 1;
            if !self.terrain[i].is_solid() {
                exits.push((i, weight));
            }
        }
        if x < self.terrain.dim.x - 1 {
            let i = idx + 1;
            if !self.terrain[i].is_solid() {
                exits.push((i, weight));
            }
        }
        if y < self.terrain.dim.y - 1 {
            let i = idx + self.terrain.dim.x as usize;
            if x > 0 && !self.terrain[i - 1].is_solid() {
                exits.push((i - 1, weight));
            }
            if !self.terrain[i].is_solid() {
                exits.push((i, weight));
            }
            if x < self.terrain.dim.x - 1 && !self.terrain[i + 1].is_solid() {
                exits.push((i + 1, weight));
            }
        }

        exits
    }
}

fn act_monsters_with_player(player: Entity, mut monsters: Vec<Entity>, state: &mut GameState) -> ChangeResult {
    let act_range_from_player2 = 32 * 32;

    let player_pos = state.world.get::<Position>(player)?.0;

    let player_loc = state.terrain.to_location(player_pos);
    let starts = vec![player_loc];
    let pather = TerrainPather::new(&state.terrain);
    let max_steps = state.terrain.dim.x + state.terrain.dim.y;
    let dijkstra_map = DijkstraMap::new(state.terrain.dim.x, state.terrain.dim.y, &starts, &pather, max_steps as f32);

    monsters.sort_by_key(|monster| {
        if let Ok(monster_pos) = state.world.get::<Position>(*monster) {
            let d = monster_pos.0 - player_pos;
            d.x * d.x + d.y * d.y
        } else {
            std::i32::MAX
        }
    });

    let mut filled = TileMap::new(state.terrain.dim, |_| false);
    for monster in monsters.iter() {
        let pos = state.world.get::<Position>(*monster)?.0;
        filled[pos] = true;
    }

    let moves: Vec<_> = monsters.iter()
        .map(|monster| {
            state.world.get::<Position>(*monster).ok().map(|monster_pos| {
                let d = monster_pos.0 - player_pos;
                let dist_to_player2 = d.x * d.x + d.y * d.y;
                if dist_to_player2 > act_range_from_player2 {
                    return None;
                }
                let mut best_score = std::f32::MAX;
                let mut best_dir = None;
                for dir in Direction::ALL.iter() {
                    let new_pos = monster_pos.0 + dir.to_point();
                    if new_pos == player_pos {
                        best_dir = Some(dir);
                        break;
                    }
                    if !filled[new_pos] {
                        let new_loc = state.terrain.to_location(new_pos);
                        let new_loc_score = dijkstra_map.map[new_loc];
                        if new_loc_score < best_score {
                            best_score = new_loc_score;
                            best_dir = Some(dir);
                        }
                    }
                }
                best_dir.map(|dir| {
                    filled[monster_pos.0] = false;
                    filled[monster_pos.0 + dir.to_point()] = true;
                    dir
                })
            }).flatten()
        })
        .collect();

    for (monster_i, monster) in monsters.iter().enumerate() {
        if let Some(dir) = moves[monster_i] {
            let pos = state.world.get::<Position>(*monster)?.0;
            if pos + dir.to_point() == player_pos {
                result_error(super::act(*monster, Action::MeleeAttack(*dir), state));
            } else {
                let _ = super::act(*monster, Action::Move(*dir), state);
            }
        }
    }

    Ok(ChangeOk)
}

fn act_monsters_without_player(monsters: Vec<Entity>, state: &mut GameState) -> ChangeResult {
    for monster in monsters {
        result_error(super::act(monster, Action::DoNothing, state));
    }
    Ok(ChangeOk)
}

pub fn act_monsters(state: &mut GameState) {
    debug!("starting monster ai");

    let monsters: Vec<_> = state.world.query::<(&IsAi, &Speed)>()
        .iter()
        .filter(|(_, (_, s))| state.turn % s.0 == 0)
        .map(|(e, _)| e)
        .collect();

    if let Some(player) = state.player {
        result_error(act_monsters_with_player(player, monsters, state))
    } else {
        result_error(act_monsters_without_player(monsters, state))
    }
}
