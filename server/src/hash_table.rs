use std::collections::LinkedList;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::borrow::Borrow;
use std::{mem, usize};

#[derive(Debug)]
pub struct HashTable<K, V> {
    buckets: Vec<LinkedList<(K,V)>>,
    
    /// number of elements in the hash_table
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq + Clone + std::fmt::Debug,
    V: Clone,
{

    pub fn new(capacity: usize) -> Self {
        HashTable {
            buckets: vec![LinkedList::new(); capacity],
            size: 0
        }
    }

    /// returns an index
    pub fn hash<Q>(&self, key: &Q) -> usize 
    where
    K: Borrow<Q>,
    Q: Hash + ?Sized
    {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        
        s.finish() as usize % self.buckets.len()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let idx: usize = self.hash(&k);
        let bucket: &mut LinkedList<(K, V)> = &mut self.buckets[idx];

        for (old_key, old_value) in bucket.iter_mut() {
            if k == *old_key {
                return Some(mem::replace(old_value, v));
            }
        }

        // hash table did not have this key present
        bucket.push_back((k, v));
        self.size += 1;
        return None
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index: usize = self.hash(k);
        let bucket: &LinkedList<(K, V)> = &self.buckets[index];

        for (key, value) in bucket.iter() {
            if k == key.borrow() {
                return Some(value);
            }
        }
        // hash table did not have this key present
        None
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V> 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index: usize = self.hash(k);
        let bucket: &mut LinkedList<(K, V)> = &mut self.buckets[index];

        let mut idx = -1;
        for (key, _) in bucket.iter() {
            idx += 1;
            if k == key.borrow() {
                break;        
            }
        }

        // workaround 
        if idx != -1 {
            let mut split: LinkedList<(K, V)> = bucket.split_off(idx as usize);
            if let Some((_, value)) = split.pop_front() {
                bucket.append(&mut split);
                return Some(value);
            }
        }
        
        // hash table did not have this key present
        None
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

}