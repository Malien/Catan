use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

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
    pub fn from_vec(values: Vec<V>) -> Self {
        Self {
            values,
            _phantom: PhantomData,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<K, V> SingleAdjacencyList<K, V>
where
    K: TryFrom<usize>,
    <K as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn push(&mut self, value: V) -> K {
        let id = self.values.len().try_into().unwrap();
        self.values.push(value);
        id
    }
}

impl<K, V> Index<K> for SingleAdjacencyList<K, V>
where
    K: Into<usize>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.values[index.into()]
    }
}

impl<K, V> IndexMut<K> for SingleAdjacencyList<K, V>
where
    K: Into<usize>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.values[index.into()]
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
pub struct CappedRelationship<V, const CAPACITY: usize> {
    size: u8,
    values: [MaybeUninit<V>; CAPACITY],
}

#[derive(Debug)]
pub struct CappedAdjacencyList<K, V, const CAPACITY: usize> {
    values: Vec<CappedRelationship<V, CAPACITY>>,
    _phantom: PhantomData<K>,
}

impl<K, V, const CAPACITY: usize> Default for CappedAdjacencyList<K, V, CAPACITY> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}
