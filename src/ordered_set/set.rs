use std::sync::atomic::{AtomicUsize, Ordering};

use super::set_item::OrderedSetItem;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct OrderedSet {
  pub id: usize,
  items: Vec<OrderedSetItem>,
}

impl OrderedSet {
  /// Instantiate new OrderedSet
  pub fn new() -> OrderedSet {
    OrderedSet {
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      items: Vec::new(),
    }
  }

  /// Returns the number of items in the Set.
  pub fn size(&self) -> usize {
    self.items.len()
  }

  pub fn add(&mut self, items: Vec<OrderedSetItem>) -> () {
    for item in items.into_iter() {
      if let Err(index) = self.items.binary_search(&item) {
        self.items.insert(index, item);
      }
    }
  }

  pub fn get_at(&self, index: usize) -> i64 {
    match self.items.get(index) {
      Some(item) => item.id as i64,
      None => -1,
    }
  }

  pub fn contains(&self, item: &OrderedSetItem) -> bool {
    self.items.contains(item)
  }

  pub fn find(&self, item: &OrderedSetItem) -> i64 {
    match self.items.binary_search(&item) {
      Ok(index) => index as i64,
      _ => -1,
    }
  }

  pub fn slice(&self, start: usize, end: usize) -> Vec<u32> {
    match self.items.get(start..end.min(self.items.len())) {
      Some(v) => v.iter().map(|item| item.id.clone()).collect(),
      None => vec![],
    }
  }

  /// Remove the last item from the Set.
  pub fn pop(&mut self) -> i64 {
    if self.items.len() > 0 {
      self.items.remove(self.items.len() - 1).id as i64
    } else {
      -1
    }
  }

  /// Remove the first item from the Set.
  pub fn shift(&mut self) -> i64 {
    if self.items.len() > 0 {
      self.items.remove(0).id as i64
    } else {
      -1
    }
  }

  pub fn remove(&mut self, items: &Vec<OrderedSetItem>) -> Vec<u32> {
    let mut removed = vec![];
    for item in items.into_iter() {
      if let Ok(index) = self.items.binary_search(&item) {
        let i = self.items.remove(index);
        removed.push(i.id)
      }
    }
    removed
  }

  pub fn remove_at(&mut self, index: usize) -> i64 {
    if self.items.len() > 0 {
      self.items.remove(index).id as i64
    } else {
      -1
    }
  }

  pub fn to_array(&self) -> Vec<u32> {
    self.items.iter().map(|item| item.id.clone()).collect()
  }

  pub fn clear(&mut self) -> () {
    self.items.clear()
  }

  pub fn union(&self, other: &Self) -> Vec<u32> {
    let mut new_set = OrderedSet::new();
    new_set.add(self.items.clone());
    new_set.add(other.items.clone());
    new_set.to_array()
  }

  pub fn difference(&self, other: &Self) -> Vec<u32> {
    let mut new_set = OrderedSet::new();
    new_set.add(self.items.clone());
    for i in 0..other.items.len() {
      let ref item = other.items.get(i).unwrap();
      if let Ok(index) = new_set.items.binary_search(item) {
        new_set.remove_at(index);
      }
    }
    new_set.to_array()
  }

  pub fn intersection(&self, other: &Self) -> Vec<u32> {
    let mut new_set = OrderedSet::new();
    for i in 0..other.items.len() {
      let item = other.items.get(i).unwrap();
      if other.items.contains(item) {
        new_set.add(vec![item.clone()]);
      }
    }
    new_set.to_array()
  }
}
