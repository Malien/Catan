use enum_map::EnumMap;

use crate::{
    adjacency_list::AdjacencyList,
    ids::{DiceMarkerID, ResourceTileID, RoadID, SettlePlaceID, TileID},
    types::{DiceMarker, HexSide, HexVertex, Player, PlayerHand, Tile}, array_vec::ArrayVec,
};

#[derive(Debug, Default)]
pub struct TileRelationships {
    pub resource: AdjacencyList<TileID, Tile>,
    pub roads: AdjacencyList<TileID, EnumMap<HexSide, RoadID>>,
    pub settle_places: AdjacencyList<TileID, EnumMap<HexVertex, SettlePlaceID>>,
}

#[derive(Debug, Default)]
pub struct RoadRelationships {
    pub settle_places: AdjacencyList<RoadID, [SettlePlaceID; 2]>,
}

#[derive(Debug, Default)]
pub struct PlayerRelationships {
    pub placed_roads: AdjacencyList<Player, Vec<RoadID>>,
    pub towns: AdjacencyList<Player, Vec<SettlePlaceID>>,
    pub settlements: AdjacencyList<Player, Vec<SettlePlaceID>>,
    pub hand: AdjacencyList<Player, PlayerHand>,
}

#[derive(Debug, Default)]
pub struct SettlePlaceRelationships {
    pub roads: AdjacencyList<SettlePlaceID, ArrayVec<RoadID, 3>>,
    // pub tiles: CappedAdjacencyList<TileID, 2, 3>
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiceMarkerRelationships {
    pub values: AdjacencyList<DiceMarkerID, DiceMarker>,
    pub place: AdjacencyList<DiceMarkerID, ResourceTileID>,
}

#[derive(Debug, Default)]
pub struct GameMap {
    pub tile: TileRelationships,
    pub road: RoadRelationships,
    pub player: PlayerRelationships,
    pub settle_place: SettlePlaceRelationships,
}
