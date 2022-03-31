#![feature(generic_const_exprs)]
#![feature(array_from_fn)]

use enum_map::{Enum, EnumMap};
use serde::Deserialize;

pub(crate) mod adjacency_list;
use adjacency_list::*;

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
    resources: EnumMap<Resource, u8>,
    settlements: u8,
    towns: u8,
    roads: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct TileID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceTileID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoadID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlePlaceID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceMarkerID(u8);

#[derive(Debug, Default)]
pub struct TileRelationships {
    pub resource: SingleAdjacencyList<TileID, Tile>,
    // pub roads: SizedAdjacencyList<TileID, RoadID, 6>,
    pub settle_places: SizedAdjacencyList<TileID, SettlePlaceID, 6>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub struct TileMap<T> {
    #[serde(default)]
    pub field: T,
    #[serde(default)]
    pub pasture: T,
    #[serde(default)]
    pub forest: T,
    #[serde(default)]
    pub mesa: T,
    #[serde(default)]
    pub mountains: T,
    #[serde(default)]
    pub desert: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct Vec2(u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum HexSide {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Harbour {
    Wheat,
    Sheep,
    Wood,
    Ore,
    Brick,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct HarbourPlacement {
    position: Vec2,
    side: HexSide,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapConfig {
    tile_bank: TileMap<u8>,
    map_size: Vec2,
    tile_placement: Vec<Vec2>,
    default_tiles: Vec<Tile>,
    #[serde(default)]
    fixed_tiles: TileMap<Vec<TileID>>,
    harbour_placement: Vec<HarbourPlacement>,
    default_harbours: Vec<Harbour>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeConfigError {
    InvalidPlayerCount(u8),
}

pub fn decode_config(config: MapConfig, player_count: u8) -> Result<GameMap, DecodeConfigError> {
    use DecodeConfigError::*;

    if !(2..=4).contains(&player_count) {
        return Err(InvalidPlayerCount(player_count));
    }

    let resource = SingleAdjacencyList::new(config.default_tiles);
    let settle_places = SizedAdjacencyList::new(vec![]);

    let tile = TileRelationships {
        resource,
        settle_places,
    };

    let map = GameMap {
        tile,
        ..Default::default()
    };

    Ok(map)
}

#[cfg(test)]
mod test {
    use crate::{
        adjacency_list::SizedAdjacencyList, decode_config, MapConfig, SettlePlaceID,
        SingleAdjacencyList, Tile, TileMap, Vec2,
    };

    #[test]
    fn decode_one_tile_map() {
        let config = MapConfig {
            tile_bank: TileMap {
                desert: 1,
                ..Default::default()
            },
            map_size: Vec2(1, 1),
            tile_placement: vec![Vec2(0, 0)],
            default_tiles: vec![Tile::Desert],
            fixed_tiles: TileMap::default(),
            harbour_placement: vec![],
            default_harbours: vec![],
        };

        let res = decode_config(config, 2).unwrap();

        assert_eq!(
            res.tile.resource,
            SingleAdjacencyList::new(vec![Tile::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            SizedAdjacencyList::new(vec![std::array::from_fn(|idx| SettlePlaceID(idx as u8))])
        );
    }
}
