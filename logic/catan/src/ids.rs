use std::num::TryFromIntError;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Hash)]
pub struct TileID(pub u8);

impl From<TileID> for usize {
    fn from(v: TileID) -> Self {
        v.0 as usize
    }
}

impl TryFrom<usize> for TileID {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(TileID)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceTileID(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoadID(pub u16);

impl From<RoadID> for usize {
    fn from(v: RoadID) -> Self {
        v.0 as usize
    }
}

impl TryFrom<usize> for RoadID {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(RoadID)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlePlaceID(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceMarkerID(pub u8);
