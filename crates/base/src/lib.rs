//! Core functionality for Feather. This crate primarily
//! implements or reexports essential data structures, such as:
//! * Inventories
//! * The block ID system
//! * The chunk data structure

use std::time::Duration;

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

pub mod anvil;
pub mod chunk;
pub mod inventory;
pub mod metadata;
mod positions;
pub mod text;
mod world;

pub use blocks::*;
pub use chunk::{Chunk, ChunkSection, CHUNK_HEIGHT, CHUNK_WIDTH};
pub use generated::{Area, Biome, EntityKind, Item, ItemStack, Particle, Window};
#[doc(inline)]
pub use metadata::EntityMetadata;
pub use positions::*;
pub use text::{deserialize_text, Text};

/// Number of updates (ticks) to do per second.
pub const TPS: u32 = 20;
/// The number of milliseconds per tick.
pub const TICK_MILLIS: u32 = 1000 / TPS;
/// The duration of a tick.
pub const TICK_DURATION: Duration = Duration::from_millis(TICK_MILLIS as u64);

/// Default port for Minecraft servers.
pub const DEFAULT_PORT: u16 = 25565;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    FromPrimitive,
    ToPrimitive,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

/// A profile property, which stores metadata
/// for some player's account. This is usually
/// used to store skin data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}
