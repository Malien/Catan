use std::{
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use array_vec::ArrayVec;
use enum_map::{enum_map, EnumMap};
use serde::Deserialize;

pub(crate) mod adjacency_list;
use adjacency_list::AdjacencyList;
pub(crate) mod ids;
use ids::*;
pub(crate) mod types;
use types::*;
pub(crate) mod relations;
use relations::*;
pub(crate) mod array_vec;

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

/// The configuration of any given map stored usually as as json file
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapConfig {
    /// The amount of different terrains in use in specified map
    tile_bank: TileMap<u8>,
    map_size: [u8; 2],
    /// Positions of all of the tiles. Index signifies TileID,
    /// while value, is the coordinated in a squared-off map
    tile_placement: Vec<[u8; 2]>,
    /// If randomization is turned off, how will the distribution
    /// of terrains lay itself.
    default_tiles: Vec<TileTerrain>,
    #[serde(default)]
    /// Terrains which should always be associated with specified TileIDs
    /// and not randomized if randomization is requested
    fixed_tiles: TileMap<Vec<TileID>>,
    /// The positions of the harbours and their rotation within specified
    /// tile. The index signifies HarborID, while the value contains the
    /// coordinate within which the harbour is places as well a the side
    /// to which it is attached within that tile.
    harbour_placement: Vec<HarbourPlacement>,
    /// If randomization is turned off, how will the distribution
    /// of harbours lay itself.
    default_harbours: Vec<Harbour>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeConfigError {
    InvalidPlayerCount(u8),
}

/// Given map config, randomization preference, and player count, generate game state.
pub fn decode_config(config: MapConfig, player_count: u8) -> Result<GameState, DecodeConfigError> {
    use DecodeConfigError::*;

    if !(2..=4).contains(&player_count) {
        return Err(InvalidPlayerCount(player_count));
    }

    // Until randomization is implemented, just provide the default distribution of terrains.
    let resource = AdjacencyList::from_vec(config.default_tiles);
    let TileTraversalResult {
        tile_settle_places,
        tile_roads,
        road_settle_places,
        settle_places_count,
    } = traverse_tiles(config.map_size, config.tile_placement);

    let tile_relations = TileEntities {
        resource,
        roads: tile_roads,
        settle_places: tile_settle_places,
    };

    let settle_relations = SettlePlaceEntities {
        roads: derive_settle_place_roads_relations(&road_settle_places, settle_places_count),
    };

    let road_relations = RoadEntities {
        settle_places: road_settle_places,
    };

    let map = GameState {
        tile: tile_relations,
        road: road_relations,
        settle_place: settle_relations,
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

struct TileTraversalResult {
    tile_settle_places: TileRelations<EnumMap<HexVertex, SettlePlaceID>>,
    tile_roads: TileRelations<EnumMap<HexSide, RoadID>>,
    road_settle_places: RoadRelations<[SettlePlaceID; 2]>,
    settle_places_count: u16,
}

/// Do a graph traversal (BSF) of tiles, while filling in the relations between tiles, roads and settle places
fn traverse_tiles(map_size: [u8; 2], tile_placement: Vec<[u8; 2]>) -> TileTraversalResult {
    use VisitStatus::*;

    let mut queue = VecDeque::new();
    queue.push_back((TileID(0), tile_placement[0]));

    let map_2d = derive_2d_map(map_size, tile_placement);

    let mut processed_tiles = HashSet::new();
    let mut settle_places_count = 0;
    // Relationships between tiles and settle places located at the vertexes of said tile
    let mut tile_settle_places = TileRelations::<EnumMap<HexVertex, SettlePlaceID>>::new();
    // Relationships between tiles and roads located at the sides of said tile
    let mut tile_roads = TileRelations::<EnumMap<HexSide, RoadID>>::new();
    // Relationships between roads and the settle places it is connecting.
    let mut road_settle_places = RoadRelations::<[SettlePlaceID; 2]>::new();

    // While queue of tiles to be processed is not empty
    while let Some((tile_id, pos)) = queue.pop_front() {
        // If tile is already processed (HashSet::insert returns true if value wasn't in the set),
        // skip processing it
        let not_processed = processed_tiles.insert(tile_id);
        if !not_processed {
            continue;
        }

        // For each neighbor tile might have, determine the status of said tile.
        // Either processed, not visited, or not a tile completely.
        let neighbor_status = neighbor_positions(pos).map(|_, pos| match map_2d[pos] {
            Some(tile_id) if processed_tiles.contains(&tile_id) => Processed(tile_id),
            Some(tile_id) => NotVisited(tile_id, pos),
            None => NotATile,
        });

        // For each neighboring side, if the neighboring tile is not present, or is not processed,
        // create a new (monotonically increasing) SettlePlaceID. Buf if a neighbor was already
        // processed copy it's settle place id as ours (with respects to the correct correlation
        // of vertexes).
        let settle_places =
            settle_places_lookup().map(|_, [(a_side, a_vert), (b_side, b_vert)]| {
                if let Processed(neighbor_id) = neighbor_status[a_side] {
                    tile_settle_places[neighbor_id][a_vert]
                } else if let Processed(neighbor_id) = neighbor_status[b_side] {
                    tile_settle_places[neighbor_id][b_vert]
                } else {
                    let id = SettlePlaceID(settle_places_count);
                    settle_places_count += 1;
                    id
                }
            });

        // Do the same trick (where we copy existing road IDs from our already
        // processed neighbors) as with the settle places, to the roads.
        // But, if the road were not previously constructed, also fill in 
        // relationship between road and two settle places it connects.
        let roads = neighbor_status.map(|side, status| {
            if let Processed(id) = status {
                tile_roads[id][side.opposite()]
            } else {
                let connected_settle_places =
                    side.connected_vertices().map(|vert| settle_places[vert]);
                road_settle_places.push(connected_settle_places)
            }
        });

        tile_settle_places.push(settle_places);
        tile_roads.push(roads);

        // Add to the queue all of the neighbors we haven't processed yet
        queue.extend(
            neighbor_status
                .into_values()
                .filter_map(VisitStatus::not_visited),
        )
    }

    TileTraversalResult {
        tile_settle_places,
        tile_roads,
        road_settle_places,
        settle_places_count,
    }
}

/// Given the relationships of RoadID -> SettlePlaceID produce the 
/// inverse relationships of kind SettlePlaceID -> RoadID
fn derive_settle_place_roads_relations(
    road_settle_places: &AdjacencyList<RoadID, [SettlePlaceID; 2]>,
    settle_places_count: u16,
) -> AdjacencyList<SettlePlaceID, ArrayVec<RoadID, 3>> {
    // Create AdjacencyList of empty vecs, ot be filled in
    let mut settle_place_roads = AdjacencyList::from_vec(
        std::iter::repeat_with(ArrayVec::new)
            .take(settle_places_count as usize)
            .collect(),
    );

    for (road, [settle_place_a, settle_place_b]) in road_settle_places {
        settle_place_roads[*settle_place_a].push(road);
        settle_place_roads[*settle_place_b].push(road);
    }

    settle_place_roads
}

/// Given the size of the map and the positions of tiles within, produce
/// 2D Matrix of map size, where each value is either the id of a tile
/// in the position, or nothing, if no such tile is located there
fn derive_2d_map([width, height]: [u8; 2], tile_placement: Vec<[u8; 2]>) -> Matrix<Option<TileID>> {
    let width = width as usize;
    let height = height as usize;
    let mut map_2d = Matrix::from_vec(vec![None; width * height], width);
    for (idx, pos) in tile_placement.into_iter().enumerate() {
        map_2d[pos] = Some(TileID(idx.try_into().unwrap()))
    }
    map_2d
}

/// Just a small simple convenience struct which allows indexing
/// it with pairs of u8's which represent 2d coordinates
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

/// The mapping of tile vertex to the pair of neighboring sides which may
/// contain the same vertex, but in a different position within their geometry
fn settle_places_lookup() -> EnumMap<HexVertex, [(HexSide, HexVertex); 2]> {
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

/// Given the coordinate of the tile, produce the set of neighbor coordinates 
/// with the correlation as which side it is neighboring with.
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

#[cfg(test)]
mod test {
    use enum_map::enum_map;

    use crate::{
        array_vec::array_vec, decode_config, ids::RoadID, types::HexSide, AdjacencyList, HexVertex,
        MapConfig, SettlePlaceID, TileMap, TileTerrain,
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
            default_tiles: vec![TileTerrain::Desert],
            fixed_tiles: TileMap::default(),
            harbour_placement: vec![],
            default_harbours: vec![],
        };

        let res = decode_config(config, 2).unwrap();

        assert_eq!(
            res.tile.resource,
            AdjacencyList::from_vec(vec![TileTerrain::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            AdjacencyList::from_vec(vec![enum_map! {
                HexVertex::NorthWest => SettlePlaceID(1),
                HexVertex::North => SettlePlaceID(0),
                HexVertex::NorthEast => SettlePlaceID(2),
                HexVertex::SouthEast => SettlePlaceID(4),
                HexVertex::South => SettlePlaceID(5),
                HexVertex::SouthWest => SettlePlaceID(3),
            }])
        );

        assert_eq!(
            res.tile.roads,
            AdjacencyList::from_vec(vec![enum_map! {
                HexSide::NorthWest => RoadID(0),
                HexSide::NorthEast => RoadID(1),
                HexSide::West => RoadID(2),
                HexSide::East => RoadID(3),
                HexSide::SouthWest => RoadID(4),
                HexSide::SouthEast => RoadID(5),
            }])
        );

        assert_eq!(
            res.road.settle_places,
            AdjacencyList::from_vec(vec![
                [SettlePlaceID(1), SettlePlaceID(0)],
                [SettlePlaceID(0), SettlePlaceID(2)],
                [SettlePlaceID(3), SettlePlaceID(1)],
                [SettlePlaceID(2), SettlePlaceID(4)],
                [SettlePlaceID(5), SettlePlaceID(3)],
                [SettlePlaceID(4), SettlePlaceID(5)],
            ])
        );

        assert_eq!(
            res.settle_place.roads,
            AdjacencyList::from_vec(vec![
                array_vec![RoadID(0), RoadID(1)],
                array_vec![RoadID(0), RoadID(2)],
                array_vec![RoadID(1), RoadID(3)],
                array_vec![RoadID(2), RoadID(4)],
                array_vec![RoadID(3), RoadID(5)],
                array_vec![RoadID(4), RoadID(5)],
            ])
        );
    }

    #[test]
    fn decode_three_tile_map() {
        let config = MapConfig {
            tile_bank: TileMap {
                desert: 3,
                ..Default::default()
            },
            map_size: [4, 4],
            tile_placement: vec![[1, 1], [2, 1], [2, 2]],
            default_tiles: vec![TileTerrain::Desert, TileTerrain::Desert, TileTerrain::Desert],
            fixed_tiles: TileMap::default(),
            harbour_placement: vec![],
            default_harbours: vec![],
        };

        let res = decode_config(config, 2).unwrap();

        assert_eq!(
            res.tile.resource,
            AdjacencyList::from_vec(vec![TileTerrain::Desert, TileTerrain::Desert, TileTerrain::Desert])
        );

        assert_eq!(
            res.tile.settle_places,
            AdjacencyList::from_vec(vec![
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

        assert_eq!(
            res.tile.roads,
            AdjacencyList::from_vec(vec![
                enum_map! {
                    HexSide::NorthWest => RoadID(0),
                    HexSide::NorthEast => RoadID(1),
                    HexSide::West => RoadID(2),
                    HexSide::East => RoadID(3),
                    HexSide::SouthWest => RoadID(4),
                    HexSide::SouthEast => RoadID(5)
                },
                enum_map! {
                    HexSide::NorthWest => RoadID(6),
                    HexSide::NorthEast => RoadID(7),
                    HexSide::West => RoadID(3),
                    HexSide::East => RoadID(8),
                    HexSide::SouthWest => RoadID(9),
                    HexSide::SouthEast => RoadID(10)
                },
                enum_map! {
                    HexSide::NorthWest => RoadID(5),
                    HexSide::NorthEast => RoadID(9),
                    HexSide::West => RoadID(11),
                    HexSide::East => RoadID(12),
                    HexSide::SouthWest => RoadID(13),
                    HexSide::SouthEast => RoadID(14)
                }
            ])
        );
    }
}
