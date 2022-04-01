use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A data structure meant to be used for representing mappings from K -> V.
/// 
/// Values are stored contiguously in memory in a Vec<V>. The indexes of the said
/// Vec are the keys representing association of K -> V.
/// 
/// Keys must be convertible from and to usize.
#[derive(Clone, PartialEq, Eq)]
pub struct AdjacencyList<K, V> {
    values: Vec<V>,
    _phantom: PhantomData<K>,
}

impl<K, V> std::fmt::Debug for AdjacencyList<K, V>
where
    K: TryFrom<usize>,
    K::Error: std::fmt::Debug,
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<K, V> Default for AdjacencyList<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V> AdjacencyList<K, V> {
    /// Create new AdjacencyList with values from the vec. Each values is assigned
    /// an key in relation to the index of such value in th vec.
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

impl<K, V> AdjacencyList<K, V>
where
    K: TryFrom<usize>,
    <K as TryFrom<usize>>::Error: std::fmt::Debug,
{
    /// Push a new mapping K -> V, where K is the new key of the value.
    pub fn push(&mut self, value: V) -> K {
        let id = self.values.len().try_into().unwrap();
        self.values.push(value);
        id
    }
}

impl<K, V> Index<K> for AdjacencyList<K, V>
where
    K: Into<usize>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.values[index.into()]
    }
}

impl<K, V> IndexMut<K> for AdjacencyList<K, V>
where
    K: Into<usize>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.values[index.into()]
    }
}

pub struct Iter<'a, K, V> {
    idx: usize,
    values: &'a Vec<V>,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> std::iter::IntoIterator for &'a AdjacencyList<K, V>
where
    K: TryFrom<usize>,
    K::Error: std::fmt::Debug,
{
    type Item = (K, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            idx: 0,
            values: &self.values,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: TryFrom<usize>,
    K::Error: std::fmt::Debug,
{
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.values.get(self.idx).map(|values| {
            let key = self.idx.try_into().unwrap();
            self.idx += 1;
            (key, values)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.values.len(), Some(self.values.len()))
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V>
where
    K: TryFrom<usize>,
    K::Error: std::fmt::Debug,
{
    fn len(&self) -> usize {
        self.values.len()
    }
}
