use enum_map::{Enum, EnumMap};
use serde::Deserialize;

use crate::ids::PlayerID;

/// The five fundamental resources in the game of Catan
#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq)]
pub enum Resource {
    Wheat,
    Sheep,
    Wood,
    Brick,
    Ore,
}

/// The six tile terrains in the game of Catan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TileTerrain {
    Field,
    Pasture,
    Forest,
    Mesa,
    Mountains,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlePlace {
    Settlement(PlayerID),
    Town(PlayerID),
    Empty,
}

/// Markers put on top of the Catan tiles signifying possible
/// outcomes of a two dice roll (Except for seven, which is 
/// reserved for robbers actions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiceMarker {
    Two,
    Three,
    Four,
    Five,
    Six,
    // Seven is for robbers
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

/// Current resources, dev cards and objects left to place of a given player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerHand {
    pub resources: EnumMap<Resource, u8>,
    pub settlements: u8,
    pub towns: u8,
    pub roads: u8,
}

/// All of the sides of a hexagonal tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Enum)]
pub enum HexSide {
    #[serde(rename = "nw")]
    NorthWest,
    #[serde(rename = "ne")]
    NorthEast,
    #[serde(rename = "w")]
    West,
    #[serde(rename = "e")]
    East,
    #[serde(rename = "sw")]
    SouthWest,
    #[serde(rename = "se")]
    SouthEast,
}

impl HexSide {
    /// Get the orthogonal side of the hexagonal tile
    pub fn opposite(self) -> Self {
        use HexSide::*;
        match self {
            NorthWest => SouthEast,
            NorthEast => SouthWest,
            West => East,
            East => West,
            SouthWest => NorthEast,
            SouthEast => NorthWest,
        }
    }

    /// Each given side connects two vertexes together. This function gives you
    /// which two concrete vertexes the specified side connects.
    pub fn connected_vertices(self) -> [HexVertex; 2] {
        match self {
            HexSide::NorthWest => [HexVertex::NorthWest, HexVertex::North],
            HexSide::NorthEast => [HexVertex::North, HexVertex::NorthEast],
            HexSide::East => [HexVertex::NorthEast, HexVertex::SouthEast],
            HexSide::SouthEast => [HexVertex::SouthEast, HexVertex::South],
            HexSide::SouthWest => [HexVertex::South, HexVertex::SouthWest],
            HexSide::West => [HexVertex::SouthWest, HexVertex::NorthWest],
        }
    }
}

/// All of the vertexes of a hexagonal tile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum HexVertex {
    North,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Harbour {
    Wheat,
    Sheep,
    Wood,
    Ore,
    Brick,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct HarbourPlacement {
    position: [u8; 2],
    side: HexSide,
}
