use enum_map::EnumMap;

use crate::{
    adjacency_list::AdjacencyList,
    array_vec::ArrayVec,
    ids::{DiceMarkerID, ResourceTileID, RoadID, SettlePlaceID, TileID, PlayerID},
    types::{DiceMarker, HexSide, HexVertex, PlayerHand, TileTerrain},
};

pub type TileRelations<T> = AdjacencyList<TileID, T>;

/// All of the properties of ALL Tile entities stored as a set of
/// relationships to all other entities.
#[derive(Debug, Default)]
pub struct TileEntities {
    pub resource: TileRelations<TileTerrain>,
    pub roads: TileRelations<EnumMap<HexSide, RoadID>>,
    pub settle_places: TileRelations<EnumMap<HexVertex, SettlePlaceID>>,
}

pub type RoadRelations<T> = AdjacencyList<RoadID, T>;

/// All of the properties of ALL Road entities stored as a set of
/// relationships to all other entities.
#[derive(Debug, Default)]
pub struct RoadEntities {
    pub settle_places: RoadRelations<[SettlePlaceID; 2]>,
}

pub type PlayerRelations<T> = AdjacencyList<PlayerID, T>;

/// All of the properties of ALL Player entities stored as a set of
/// relationships to all other entities.
#[derive(Debug, Default)]
pub struct PlayerEntities {
    pub placed_roads: PlayerRelations<Vec<RoadID>>,
    pub towns: PlayerRelations<Vec<SettlePlaceID>>,
    pub settlements: PlayerRelations<Vec<SettlePlaceID>>,
    pub hand: PlayerRelations<PlayerHand>,
}

pub type SettleRelations<T> = AdjacencyList<SettlePlaceID, T>;

/// All of the properties of ALL SettlePlaces entities stored as a set of
/// relationships to all other entities.
#[derive(Debug, Default)]
pub struct SettlePlaceEntities {
    pub roads: SettleRelations<ArrayVec<RoadID, 3>>,
    // pub tiles: CappedAdjacencyList<TileID, 2, 3>
}

pub type DiceMarkerRelations<T> = AdjacencyList<DiceMarkerID, T>;

/// All of the properties of ALL DiceMarker entities stored as a set of
/// relationships to all other entities.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiceMarkerEntities {
    pub values: DiceMarkerRelations<DiceMarker>,
    pub place: DiceMarkerRelations<ResourceTileID>,
}

/// The current state of the game, containing all of the relationships
/// between game objects and players
#[derive(Debug, Default)]
pub struct GameState {
    pub tile: TileEntities,
    pub road: RoadEntities,
    pub player: PlayerEntities,
    pub settle_place: SettlePlaceEntities,
}
