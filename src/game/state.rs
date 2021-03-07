use std::io::{Read, Write};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use hecs::{World, Entity};
use crate::tilemap::TileMap;
use super::stuff::Stuff;
use super::components::ComponentSerializationContext;

pub struct GameState {
    pub world: World,
    pub stuff: TileMap<Stuff>,
    pub player: Option<Entity>
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
    pub stuff: &'a TileMap<Stuff>,
    pub player: Option<Entity>
}

#[derive(Deserialize)]
struct LoadGameState {
    pub world: LoadWorld,
    pub stuff: TileMap<Stuff>,
    pub player: Option<Entity>
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
            stuff: &state.stuff,
            player: state.player
        }
    }
}

impl LoadGameState {
    fn to_game_state(self) -> GameState {
        GameState {
            world: self.world.world,
            stuff: self.stuff,
            player: self.player
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
