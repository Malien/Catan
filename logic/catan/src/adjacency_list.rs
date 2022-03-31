use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleAdjacencyList<K, V> {
    values: Vec<V>,
    _phantom: PhantomData<K>,
}

impl<K, V> Default for SingleAdjacencyList<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V> SingleAdjacencyList<K, V> {
    pub fn new(values: Vec<V>) -> Self {
        Self {
            values,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HSparseAdjacencyList<K, V> {
    values: Vec<Vec<V>>,
    _phantom: PhantomData<K>,
}

impl<K, V> Default for HSparseAdjacencyList<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizedAdjacencyList<K, V, const SIZE: usize> {
    values: Vec<[V; SIZE]>,
    _phantom: PhantomData<K>,
}

impl<K, V, const SIZE: usize> Default for SizedAdjacencyList<K, V, SIZE> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V, const SIZE: usize> SizedAdjacencyList<K, V, SIZE> {
    pub fn new(values: Vec<[V; SIZE]>) -> Self {
        Self {
            values,
            _phantom: PhantomData,
        }
    }
}


#[derive(Debug)]
/// SAFETY: CAPACITY should not exceed 255 (does not overflow u8)
pub struct CappedRelationship<V, const CAPACITY: /* u8 */ usize>
// where
//     [(); CAPACITY as usize]: ,
{
    size: u8,
    values: [MaybeUninit<V>; CAPACITY /* as usize */],
}

#[derive(Debug)]
pub struct CappedAdjacencyList<K, V, const CAPACITY: /* u8 */ usize>
// where
//     [(); CAPACITY as usize]: ,
{
    values: Vec<CappedRelationship<V, CAPACITY>>,
    _phantom: PhantomData<K>,
}

impl<K, V, const CAPACITY: /* u8 */ usize> Default for CappedAdjacencyList<K, V, CAPACITY>
// where
//     [(); CAPACITY as usize]: ,
{
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}
