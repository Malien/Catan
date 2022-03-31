use std::collections::{HashSet, VecDeque};

use enum_map::{enum_map, Enum, EnumMap};
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
    resources: EnumMap<Resource, u8>,
    settlements: u8,
    towns: u8,
    roads: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Hash)]
pub struct TileID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceTileID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoadID(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlePlaceID(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceMarkerID(u8);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Enum)]
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
    position: [u8; 2],
    side: HexSide,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapConfig {
    tile_bank: TileMap<u8>,
    map_size: [u8; 2],
    tile_placement: Vec<[u8; 2]>,
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

#[derive(Debug, Clone, Copy)]
enum VisitStatus {
    Visited(TileID),
    NotVisited(TileID, [u8; 2]),
    NotATile,
}

pub fn decode_config(config: MapConfig, player_count: u8) -> Result<GameMap, DecodeConfigError> {
    use DecodeConfigError::*;
    use VisitStatus::*;

    if !(2..=4).contains(&player_count) {
        return Err(InvalidPlayerCount(player_count));
    }

    let resource = SingleAdjacencyList::new(config.default_tiles);

    let mut queue = VecDeque::new();
    queue.push_back((config.tile_placement[0], TileID(0)));
    let [width, height] = config.map_size;
    let width = width as usize;
    let height = height as usize;
    let mut map_2d = vec![None; width * height];

    for (idx, [x, y]) in config.tile_placement.into_iter().enumerate() {
        map_2d[(x as usize) + (y as usize) * width] = Some(TileID(idx as u8))
    }

    let mut visited_tiles = HashSet::new();
    let mut settle_places = vec![];
    let mut tile_settle_places: Vec<EnumMap<HexVertex, SettlePlaceID>> = vec![];

    while let Some((pos, tile_id)) = queue.pop_front() {
        if visited_tiles.contains(&tile_id) {
            continue;
        }
        visited_tiles.insert(tile_id);
        let neighbor_positions = neighbor_positions(pos);
        let neighbor_ids = neighbor_positions.map(|_, neighbor_pos| {
            match map_2d[(neighbor_pos[0] as usize) + (neighbor_pos[1] as usize) * width] {
                Some(neighbor_id) if visited_tiles.contains(&neighbor_id) => Visited(neighbor_id),
                Some(neighbor_id) => NotVisited(neighbor_id, neighbor_pos),
                None => NotATile,
            }
        });
        let settle_places = enum_map! {
            HexVertex::North => {
                match (neighbor_ids[HexSide::NorthWest], neighbor_ids[HexSide::NorthEast]) {
                    (Visited(tile_id), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::SouthEast]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::SouthWest]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
            HexVertex::NorthEast => {
                match (neighbor_ids[HexSide::NorthEast], neighbor_ids[HexSide::East]) {
                    (Visited(tile_id, ), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::South]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::NorthWest]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
            HexVertex::SouthEast => {
                match (neighbor_ids[HexSide::East], neighbor_ids[HexSide::SouthEast]) {
                    (Visited(tile_id), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::SouthWest]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::North]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
            HexVertex::South => {
                match (neighbor_ids[HexSide::SouthEast], neighbor_ids[HexSide::SouthWest]) {
                    (Visited(tile_id), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::SouthWest]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::North]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
            HexVertex::SouthWest => {
                match (neighbor_ids[HexSide::SouthWest], neighbor_ids[HexSide::West]) {
                    (Visited(tile_id), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::North]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::SouthEast]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
            HexVertex::NorthWest => {
                match (neighbor_ids[HexSide::West], neighbor_ids[HexSide::NorthWest]) {
                    (Visited(tile_id), _) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::NorthEast]
                    }
                    (_, Visited(tile_id)) => {
                        tile_settle_places[tile_id.0 as usize][HexVertex::South]
                    }
                    _ => alloc_settle_place(&mut settle_places),
                }
            },
        };
        tile_settle_places.push(settle_places);
        for (_, neighbor_visit_status) in neighbor_ids {
            if let NotVisited(neighbor_id, neighbor_pos) = neighbor_visit_status {
                queue.push_back((neighbor_pos, neighbor_id))
            }
        }
    }

    let tile = TileRelationships {
        resource,
        settle_places: SingleAdjacencyList::new(tile_settle_places),
    };

    let map = GameMap {
        tile,
        ..Default::default()
    };

    Ok(map)
}

fn neighbor_positions([x, y]: [u8; 2]) -> EnumMap<HexSide, [u8; 2]> {
    use HexSide::*;
    if y % 2 == 0 {
        enum_map! {
            NorthWest => [x-1, y-1],
            NorthEast => [x,y-1],
            West => [x-1, y],
            East => [x+1, y],
            SouthWest => [x-1, y+1],
            SouthEast => [x, y+1],
        }
    } else {
        enum_map! {
            NorthWest => [x, y-1],
            NorthEast => [x+1,y-1],
            West => [x-1, y],
            East => [x+1, y],
            SouthWest => [x, y+1],
            SouthEast => [x+1, y+1],
        }
    }
}

fn alloc_settle_place(settle_places: &mut Vec<SettlePlace>) -> SettlePlaceID {
    let id = SettlePlaceID(settle_places.len().try_into().unwrap());
    settle_places.push(SettlePlace::Empty);
    id
}

#[cfg(test)]
mod test {
    use enum_map::enum_map;

    use crate::{
        adjacency_list::SizedAdjacencyList, decode_config, HexVertex, MapConfig, SettlePlaceID,
        SingleAdjacencyList, Tile, TileMap,
    };

    #[inline]
    fn array_from_fn<F, T, const N: usize>(mut cb: F) -> [T; N]
    where
        F: FnMut(usize) -> T,
    {
        let mut idx = 0;
        [(); N].map(|_| {
            let res = cb(idx);
            idx += 1;
            res
        })
    }

    #[test]
    fn decode_one_tile_map() {
        let config = MapConfig {
            tile_bank: TileMap {
                desert: 1,
                ..Default::default()
            },
            map_size: [3, 3],
            tile_placement: vec![[1, 1]],
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
            SingleAdjacencyList::new(vec![enum_map! {
                HexVertex::North => SettlePlaceID(0),
                HexVertex::NorthWest => SettlePlaceID(1),
                HexVertex::NorthEast => SettlePlaceID(2),
                HexVertex::SouthWest => SettlePlaceID(3),
                HexVertex::SouthEast => SettlePlaceID(4),
                HexVertex::South => SettlePlaceID(5),
            }])
        );
    }

    #[test]
    fn decode_tree_tile_map() {
        let config = MapConfig {
            tile_bank: TileMap {
                desert: 1,
                ..Default::default()
            },
            map_size: [4, 4],
            tile_placement: vec![[1, 1], [2, 1], [2, 2]],
            default_tiles: vec![Tile::Desert, Tile::Desert, Tile::Desert],
            fixed_tiles: TileMap::default(),
            harbour_placement: vec![],
            default_harbours: vec![],
        };

        let res = decode_config(config, 2).unwrap();

        assert_eq!(
            res.tile.resource,
            SingleAdjacencyList::new(vec![Tile::Desert, Tile::Desert, Tile::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            SingleAdjacencyList::new(vec![
                enum_map! {
                    HexVertex::North => SettlePlaceID(0),
                    HexVertex::NorthWest => SettlePlaceID(1),
                    HexVertex::NorthEast => SettlePlaceID(2),
                    HexVertex::SouthWest => SettlePlaceID(3),
                    HexVertex::SouthEast => SettlePlaceID(4),
                    HexVertex::South => SettlePlaceID(5),
                },
                enum_map! {
                    HexVertex::North => SettlePlaceID(6),
                    HexVertex::NorthWest => SettlePlaceID(2),
                    HexVertex::NorthEast => SettlePlaceID(7),
                    HexVertex::SouthWest => SettlePlaceID(4),
                    HexVertex::SouthEast => SettlePlaceID(8),
                    HexVertex::South => SettlePlaceID(9),
                },
                enum_map! {
                    HexVertex::North => SettlePlaceID(4),
                    HexVertex::NorthWest => SettlePlaceID(5),
                    HexVertex::NorthEast => SettlePlaceID(9),
                    HexVertex::SouthWest => SettlePlaceID(10),
                    HexVertex::SouthEast => SettlePlaceID(11),
                    HexVertex::South => SettlePlaceID(12),
                }
            ])
        );
    }
}
