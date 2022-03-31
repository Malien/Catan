use std::{
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use enum_map::{enum_map, EnumMap};
use serde::Deserialize;

pub(crate) mod adjacency_list;
use adjacency_list::*;
pub(crate) mod ids;
use ids::*;
pub(crate) mod types;
use types::*;
pub(crate) mod relations;
use relations::*;

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

pub fn decode_config(config: MapConfig, player_count: u8) -> Result<GameMap, DecodeConfigError> {
    use DecodeConfigError::*;

    if !(2..=4).contains(&player_count) {
        return Err(InvalidPlayerCount(player_count));
    }

    let resource = SingleAdjacencyList::from_vec(config.default_tiles);
    let settle_places = traverse_tiles(config.map_size, config.tile_placement);

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

#[derive(Debug, Clone, Copy)]
enum VisitStatus {
    Processed(TileID),
    NotVisited(TileID, [u8; 2]),
    NotATile,
}

impl VisitStatus {
    fn not_visited(self) -> Option<(TileID, [u8; 2])> {
        if let Self::NotVisited(id, pos) = self {
            Some((id, pos))
        } else {
            None
        }
    }
}

fn traverse_tiles(
    map_size: [u8; 2],
    tile_placement: Vec<[u8; 2]>,
) -> SingleAdjacencyList<TileID, EnumMap<HexVertex, SettlePlaceID>> {
    use VisitStatus::*;
    let mut queue = VecDeque::new();
    queue.push_back((TileID(0), tile_placement[0]));
    let map_2d = derive_2d_map(map_size, tile_placement);
    let mut processed_tiles = HashSet::new();
    let mut settle_places_count = 0;
    let mut tile_settle_places = SingleAdjacencyList::<_, EnumMap<_, _>>::new();

    while let Some((tile_id, pos)) = queue.pop_front() {
        let not_processed = processed_tiles.insert(tile_id);
        if !not_processed {
            continue;
        }

        let neighbor_status = neighbor_positions(pos).map(|_, pos| match map_2d[pos] {
            Some(tile_id) if processed_tiles.contains(&tile_id) => Processed(tile_id),
            Some(tile_id) => NotVisited(tile_id, pos),
            None => NotATile,
        });

        let settle_places = neighbor_lookup().map(|_, [(a_side, a_vert), (b_side, b_vert)]| {
            if let Processed(id) = neighbor_status[a_side] {
                tile_settle_places[id][a_vert]
            } else if let Processed(id) = neighbor_status[b_side] {
                tile_settle_places[id][b_vert]
            } else {
                alloc_settle_place(&mut settle_places_count)
            }
        });

        tile_settle_places.push(settle_places);

        queue.extend(
            neighbor_status
                .into_values()
                .filter_map(VisitStatus::not_visited),
        )
    }
    tile_settle_places
}

fn derive_2d_map([width, height]: [u8; 2], tile_placement: Vec<[u8; 2]>) -> Matrix<Option<TileID>> {
    let width = width as usize;
    let height = height as usize;
    let mut map_2d = Matrix::from_vec(vec![None; width * height], width);
    for (idx, pos) in tile_placement.into_iter().enumerate() {
        map_2d[pos] = Some(TileID(idx.try_into().unwrap()))
    }
    map_2d
}

struct Matrix<T> {
    width: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn from_vec(data: Vec<T>, width: usize) -> Self {
        Self { width, data }
    }
}

impl<T> Index<[u8; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, [x, y]: [u8; 2]) -> &Self::Output {
        &self.data[x as usize + (y as usize) * self.width]
    }
}

impl<T> IndexMut<[u8; 2]> for Matrix<T> {
    fn index_mut(&mut self, [x, y]: [u8; 2]) -> &mut Self::Output {
        &mut self.data[x as usize + (y as usize) * self.width]
    }
}

fn neighbor_lookup() -> EnumMap<HexVertex, [(HexSide, HexVertex); 2]> {
    enum_map! {
        HexVertex::North => {[
            (HexSide::NorthWest, HexVertex::SouthEast),
            (HexSide::NorthEast, HexVertex::SouthEast)
        ]},
        HexVertex::NorthEast => {[
            (HexSide::NorthEast, HexVertex::South),
            (HexSide::East, HexVertex::NorthWest)
        ]},
        HexVertex::SouthEast => {[
            (HexSide::East, HexVertex::SouthWest),
            (HexSide::SouthEast, HexVertex::North)
        ]},
        HexVertex::South => {[
            (HexSide::SouthEast, HexVertex::SouthWest),
            (HexSide::SouthWest, HexVertex::North)
        ]},
        HexVertex::SouthWest => {[
            (HexSide::SouthWest, HexVertex::North),
            (HexSide::West, HexVertex::SouthEast)
        ]},
        HexVertex::NorthWest => {[
            (HexSide::West, HexVertex::NorthEast),
            (HexSide::NorthWest, HexVertex::South)
        ]},
    }
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

fn alloc_settle_place(settle_places_count: &mut u16) -> SettlePlaceID {
    let id = SettlePlaceID(*settle_places_count);
    *settle_places_count += 1;
    id
}

#[cfg(test)]
mod test {
    use enum_map::enum_map;

    use crate::{
        decode_config, HexVertex, MapConfig, SettlePlaceID, SingleAdjacencyList, Tile, TileMap,
    };

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
            SingleAdjacencyList::from_vec(vec![Tile::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            SingleAdjacencyList::from_vec(vec![enum_map! {
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
            SingleAdjacencyList::from_vec(vec![Tile::Desert, Tile::Desert, Tile::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            SingleAdjacencyList::from_vec(vec![
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
