use enum_map::EnumMap;

use crate::{
    adjacency_list::AdjacencyList,
    array_vec::ArrayVec,
    ids::{DiceMarkerID, ResourceTileID, RoadID, SettlePlaceID, TileID, PlayerID},
    types::{DiceMarker, HexSide, HexVertex, PlayerHand, TileType},
};

pub type TileRelations<T> = AdjacencyList<TileID, T>;

#[derive(Debug, Default)]
pub struct TileEntities {
    pub resource: TileRelations<TileType>,
    pub roads: TileRelations<EnumMap<HexSide, RoadID>>,
    pub settle_places: TileRelations<EnumMap<HexVertex, SettlePlaceID>>,
}

pub type RoadRelations<T> = AdjacencyList<RoadID, T>;

#[derive(Debug, Default)]
pub struct RoadEntities {
    pub settle_places: RoadRelations<[SettlePlaceID; 2]>,
}

pub type PlayerRelations<T> = AdjacencyList<PlayerID, T>;

#[derive(Debug, Default)]
pub struct PlayerEntities {
    pub placed_roads: PlayerRelations<Vec<RoadID>>,
    pub towns: PlayerRelations<Vec<SettlePlaceID>>,
    pub settlements: PlayerRelations<Vec<SettlePlaceID>>,
    pub hand: PlayerRelations<PlayerHand>,
}

pub type SettleRelations<T> = AdjacencyList<SettlePlaceID, T>;

#[derive(Debug, Default)]
pub struct SettlePlaceEntities {
    pub roads: SettleRelations<ArrayVec<RoadID, 3>>,
    // pub tiles: CappedAdjacencyList<TileID, 2, 3>
}

pub type DiceMarkerRelations<T> = AdjacencyList<DiceMarkerID, T>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiceMarkerEntities {
    pub values: DiceMarkerRelations<DiceMarker>,
    pub place: DiceMarkerRelations<ResourceTileID>,
}

#[derive(Debug, Default)]
pub struct GameMap {
    pub tile: TileEntities,
    pub road: RoadEntities,
    pub player: PlayerEntities,
    pub settle_place: SettlePlaceEntities,
}
