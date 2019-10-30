use std::collections::HashMap;
use std::sync::Mutex;

use super::set::OrderedSet;
use super::set_item::OrderedSetItem;

lazy_static! {
  static ref MAP_CACHE: Mutex<HashMap<usize, OrderedSet>> = Mutex::new(HashMap::new());
}

/// Instantiate a new Set.
pub fn new(items: Vec<OrderedSetItem>) -> usize {
  let mut coll = OrderedSet::new();
  coll.add(items);
  let id = coll.id.clone();
  if let Ok(mut map) = MAP_CACHE.lock() {
    map.insert(id, coll);
  }
  id
}

/// Add the given items to the Set.
pub fn add(id: usize, items: Vec<OrderedSetItem>) -> () {
  get_and_map(id, |set| set.add(items.clone()))
}

pub fn get_at(id: usize, index: usize) -> i64 {
  get_and_map(id, |set| set.get_at(index))
}

pub fn contains(id: usize, item: &OrderedSetItem) -> bool {
  get_and_map(id, |set| set.contains(item))
}

pub fn find(id: usize, item: &OrderedSetItem) -> i64 {
  get_and_map(id, |set| set.find(item))
}

pub fn slice(id: usize, start: usize, end: usize) -> Vec<u32> {
  get_and_map(id, |set| set.slice(start, end))
}

pub fn shift(id: usize) -> i64 {
  get_and_map(id, |set| set.shift())
}

pub fn pop(id: usize) -> i64 {
  get_and_map(id, |set| set.pop())
}

pub fn remove(id: usize, items: &Vec<OrderedSetItem>) -> Vec<u32> {
  get_and_map(id, |set| set.remove(items))
}

pub fn remove_at(id: usize, index: usize) -> i64 {
  get_and_map(id, |set| set.remove_at(index))
}

pub fn size(id: usize) -> usize {
  get_and_map(id, |set| set.size())
}

pub fn to_array(id: usize) -> Vec<u32> {
  get_and_map(id, |set| set.to_array())
}

/// Clear/empty the Set.
pub fn clear(id: usize) -> () {
  get_and_map(id, |set| set.clear())
}

/// Returns the set of values including all values from both of these sets.
pub fn union(id: usize, other: usize) -> Vec<u32> {
  let map = MAP_CACHE.lock().expect("Could not access the OrderedSet.");
  let set = map.get(&id).expect("Could not find the OrderedSet");
  let other = map
    .get(&other)
    .expect("Could not find the other OrderedSet");
  set.union(other)
}

// Returns the set of values that are in both of these sets.
pub fn intersection(id: usize, other: usize) -> Vec<u32> {
  let map = MAP_CACHE.lock().expect("Could not access the OrderedSet.");
  let set = map.get(&id).expect("Could not find the OrderedSet");
  let other = map
    .get(&other)
    .expect("Could not find the other OrderedSet");
  set.intersection(other)
}

// Returns the set of values that are in this set, excluding the values that are also in the other set.
pub fn difference(id: usize, other: usize) -> Vec<u32> {
  let map = MAP_CACHE.lock().expect("Could not access the OrderedSet.");
  let set = map.get(&id).expect("Could not find the OrderedSet");
  let other = map
    .get(&other)
    .expect("Could not find the other OrderedSet");
  set.difference(other)
}

/// Delete the Set from the cache.
pub fn delete(id: usize) -> () {
  let mut map = MAP_CACHE.lock().unwrap();
  map.remove(&id);
}

/// Find and execute on the given Set ID.
fn get_and_map<U, F>(id: usize, mut func: F) -> U
where
  F: FnMut(&mut OrderedSet) -> U,
{
  let mut map = MAP_CACHE.lock().expect("Could not access the OrderedSet.");
  let set = map.get_mut(&id).expect("Could not find the OrderedSet");
  func(set)
}
