use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderedSetItem(pub u32, Vec<Vec<u16>>, Vec<Vec<i16>>);
// pub struct OrderedSetItem {
//   pub id: u32,
//   equality: Vec<Vec<u16>>,
//   comparison: Vec<Vec<i16>>,
// }

impl PartialEq for OrderedSetItem {
  fn eq(&self, other: &Self) -> bool {
    self.1 == other.1
  }
}
impl Eq for OrderedSetItem {}

impl Ord for OrderedSetItem {
  fn cmp(&self, other: &Self) -> Ordering {
    self.2.cmp(&other.2)
  }
}

impl PartialOrd for OrderedSetItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
