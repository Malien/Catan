use enum_map::{Enum, EnumMap};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player(u8);

#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq)]
pub enum Resource {
    Wheat,
    Sheep,
    Wood,
    Brick,
    Ore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tile {
    Field,
    Pasture,
    Forest,
    Mesa,
    Mountains,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlePlace {
    Settlement(Player),
    Town(Player),
    Empty,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerHand {
    pub resources: EnumMap<Resource, u8>,
    pub settlements: u8,
    pub towns: u8,
    pub roads: u8,
}

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
