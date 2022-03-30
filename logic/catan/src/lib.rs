#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use enum_map::{Enum, EnumMap, MaybeUninit};

pub struct Player(u8);

#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq)]
pub enum Resource {
    Wheat,
    Sheep,
    Wood,
    Brick,
    Ore,
}

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

pub struct PlayerHand {
    resources: EnumMap<Resource, u8>,
    settlements: u8,
    towns: u8,
    roads: u8,
}

pub struct SingleAdjacencyList<K, V> {
    values: Vec<V>,
    _phantom: PhantomData<K>,
}

pub struct HSparseAdjacencyList<K, V> {
    values: Vec<Vec<V>>,
    _phantom: PhantomData<K>,
}

// pub struct VSparse

pub struct SizedAdjacencyList<K, V, const SIZE: usize> {
    values: Vec<[V; SIZE]>,
    _phantom: PhantomData<K>,
}

pub struct CappedRelationship<V, const MIN: u8, const MAX: u8>
where
    [(); MIN as usize]: ,
    [(); { MAX - MIN } as usize]: ,
{
    size: u8,
    min_values: [V; MIN as usize],
    optional_values: [MaybeUninit<V>; { MAX - MIN } as usize],
}

pub struct CappedAdjacencyList<K, V, const MIN: u8, const MAX: u8>
where
    [(); MIN as usize]: ,
    [(); { MAX - MIN } as usize]: ,
{
    values: Vec<CappedRelationship<V, MIN, MAX>>,
    _phantom: PhantomData<K>,
}

type EnumAdjacencyList<K, V> = EnumMap<K, Vec<V>>;

pub struct TileID(u8);
pub struct ResourceTileID(u8);
pub struct RoadID(u8);
pub struct SettlePlaceID(u8);
type PlayerID = Player;
pub struct DiceMarkerID(u8);

pub struct TileRelationships {
    pub resource: SingleAdjacencyList<TileID, Tile>,
    // pub roads: SizedAdjacencyList<TileID, RoadID, 6>,
    pub settle_places: SizedAdjacencyList<TileID, SettlePlaceID, 6>,
}

pub struct RoadRelationships {
    pub settle_places: SizedAdjacencyList<RoadID, SettlePlaceID, 2>,
}

pub struct PlayerRelationships {
    pub placed_roads: HSparseAdjacencyList<Player, RoadID>,
    pub towns: HSparseAdjacencyList<Player, SettlePlaceID>,
    pub settlements: HSparseAdjacencyList<Player, SettlePlaceID>,
}

pub struct SettlePlaceRelationships {
    pub roads: CappedAdjacencyList<SettlePlaceID, RoadID, 2, 3>,
    // pub tiles: CappedAdjacencyList<TileID, 2, 3>
}

pub struct DiceMarkerRelationships {
    pub values: SingleAdjacencyList<DiceMarkerID, DiceMarker>,
    pub place: SingleAdjacencyList<DiceMarkerID, ResourceTileID>,
}

pub struct GameMap {
    tile: TileRelationships,
    road: RoadRelationships,
    player: PlayerRelationships,
    settle_place: SettlePlaceRelationships,
}
