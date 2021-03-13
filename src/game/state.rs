use std::io::{Read, Write};
use enum_map::Enum;
use bracket_random::prelude::RandomNumberGenerator;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use hecs::{World, Entity};
use crate::tilemap::TileMap;
use super::terrain::Terrain;
use super::liquids::Liquid;
use super::components::ComponentSerializationContext;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum GameStatus {
    Playing,
    Won,
    Lost
}

pub struct GameState {
    pub world: World,
    pub particles_world: World,
    pub terrain: TileMap<Terrain>,
    pub liquids: TileMap<Option<Liquid>>,
    pub player: Option<Entity>,
    pub rng: RandomNumberGenerator,
    pub status: GameStatus,
    pub turn: u32
}

struct SaveWorld<'a> {
    world: &'a World
}

struct LoadWorld {
    world: World
}

#[derive(Serialize)]
struct SaveGameState<'a> {
    pub world: SaveWorld<'a>,
    pub terrain: &'a TileMap<Terrain>,
    pub liquids: &'a TileMap<Option<Liquid>>,
    pub player: Option<Entity>,
    pub rng: &'a RandomNumberGenerator,
    pub status: GameStatus,
    pub turn: u32
}

#[derive(Deserialize)]
struct LoadGameState {
    pub world: LoadWorld,
    pub terrain: TileMap<Terrain>,
    pub liquids: TileMap<Option<Liquid>>,
    pub player: Option<Entity>,
    pub rng: RandomNumberGenerator,
    pub status: GameStatus,
    pub turn: u32
}

impl GameState {
    pub fn save<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let io_write = serde_cbor::ser::IoWrite::new(writer);
        let mut serializer = serde_cbor::Serializer::new(io_write)
            .packed_format();
        self.serialize(&mut serializer)
            .map_err(|e| {
                let msg = format!("serialization failed: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, msg)
            })?;
        Ok(())
    }

    pub fn load<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
        let state = GameState::deserialize(&mut deserializer)
            .map_err(|e| {
                let msg = format!("component deserialization failed: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, msg)
            })?;
        Ok(state)
    }
}

impl <'a> SaveGameState<'a> {
    fn new(state: &'a GameState) -> Self {
        Self {
            world: SaveWorld { world: &state.world },
            terrain: &state.terrain,
            liquids: &state.liquids,
            player: state.player,
            rng: &state.rng,
            status: state.status,
            turn: state.turn
        }
    }
}

impl LoadGameState {
    fn to_game_state(self) -> GameState {
        GameState {
            world: self.world.world,
            particles_world: World::new(),
            terrain: self.terrain,
            liquids: self.liquids,
            player: self.player,
            rng: self.rng,
            status: self.status,
            turn: self.turn
        }
    }
}

impl Serialize for SaveWorld<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        hecs::serialize::serialize(self.world, &mut ComponentSerializationContext, serializer)
    }
}

impl <'de> Deserialize<'de> for LoadWorld {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let world = hecs::serialize::deserialize(&mut ComponentSerializationContext, deserializer)?;
        Ok(LoadWorld { world })
    }
}

impl Serialize for GameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let save_state = SaveGameState::new(&self);
        save_state.serialize(serializer)
    }
}

impl <'de> Deserialize<'de> for GameState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let load_state = LoadGameState::deserialize(deserializer)?;
        Ok(load_state.to_game_state())
    }
}
