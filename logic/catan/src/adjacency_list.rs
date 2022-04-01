use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjacencyList<K, V> {
    values: Vec<V>,
    _phantom: PhantomData<K>,
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
