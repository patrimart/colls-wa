use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderedSetItem {
  pub id: u32,
  equality: Vec<Vec<u16>>,
  comparison: Vec<Vec<i16>>,
}

impl PartialEq for OrderedSetItem {
  fn eq(&self, other: &Self) -> bool {
    self.equality == other.equality
  }
}
impl Eq for OrderedSetItem {}

impl Ord for OrderedSetItem {
  fn cmp(&self, other: &Self) -> Ordering {
    self.comparison.cmp(&other.comparison)
  }
}

impl PartialOrd for OrderedSetItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
