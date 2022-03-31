use enum_map::EnumMap;

use crate::{adjacency_list::{SingleAdjacencyList, SizedAdjacencyList, HSparseAdjacencyList, CappedAdjacencyList}, ids::{TileID, SettlePlaceID, RoadID, DiceMarkerID, ResourceTileID}, types::{Tile, HexVertex, Player, PlayerHand, DiceMarker}};

#[derive(Debug, Default)]
pub struct TileRelationships {
    pub resource: SingleAdjacencyList<TileID, Tile>,
    // pub roads: SizedAdjacencyList<TileID, RoadID, 6>,
    pub settle_places: SingleAdjacencyList<TileID, EnumMap<HexVertex, SettlePlaceID>>,
}

#[derive(Debug, Default)]
pub struct RoadRelationships {
    pub settle_places: SizedAdjacencyList<RoadID, SettlePlaceID, 2>,
}

#[derive(Debug, Default)]
pub struct PlayerRelationships {
    pub placed_roads: HSparseAdjacencyList<Player, RoadID>,
    pub towns: HSparseAdjacencyList<Player, SettlePlaceID>,
    pub settlements: HSparseAdjacencyList<Player, SettlePlaceID>,
    pub hand: SingleAdjacencyList<Player, PlayerHand>,
}

#[derive(Debug, Default)]
pub struct SettlePlaceRelationships {
    pub roads: CappedAdjacencyList<SettlePlaceID, RoadID, 3>,
    // pub tiles: CappedAdjacencyList<TileID, 2, 3>
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiceMarkerRelationships {
    pub values: SingleAdjacencyList<DiceMarkerID, DiceMarker>,
    pub place: SingleAdjacencyList<DiceMarkerID, ResourceTileID>,
}

#[derive(Debug, Default)]
pub struct GameMap {
    pub tile: TileRelationships,
    pub road: RoadRelationships,
    pub player: PlayerRelationships,
    pub settle_place: SettlePlaceRelationships,
}
