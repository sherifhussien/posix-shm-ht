use std::collections::LinkedList;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
pub struct HashTable<K, V> {
    buckets: Vec<LinkedList<(K,V)>>,
    
    /// number of elements in the hash_table
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{

    pub fn new(capacity: usize) -> Self {
        HashTable {
            buckets: vec![LinkedList::new(); capacity],
            size: 0
        }
    }

    /// returns an index
    pub fn hash(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        
        s.finish() as usize % self.buckets.len()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

}