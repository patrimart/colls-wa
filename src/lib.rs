use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod ordered_set;

#[wasm_bindgen]
pub fn ordered_set_new(val: &JsValue) -> usize {
  let items: Vec<ordered_set::set_item::OrderedSetItem> = val
    .into_serde()
    .expect("Paramerer should be an OrderedSetItem.");
  ordered_set::public_js::new(items)
}

#[wasm_bindgen]
pub fn ordered_set_add(id_js: &JsValue, items_js: &JsValue) -> () {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let items: Vec<ordered_set::set_item::OrderedSetItem> = items_js
    .into_serde()
    .expect("Paramerer should be an OrderedSetItem.");
  ordered_set::public_js::add(id, items)
}

#[wasm_bindgen]
pub fn ordered_set_get_at(id_js: &JsValue, index_js: &JsValue) -> i64 {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let index: usize = index_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::get_at(id, index)
}

#[wasm_bindgen]
pub fn ordered_set_contains(id_js: &JsValue, item_js: &JsValue) -> bool {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let item: ordered_set::set_item::OrderedSetItem = item_js
    .into_serde()
    .expect("Paramerer should be an OrderedSetItem.");
  ordered_set::public_js::contains(id, &item)
}

#[wasm_bindgen]
pub fn ordered_set_find(id_js: &JsValue, item_js: &JsValue) -> i64 {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let item: ordered_set::set_item::OrderedSetItem = item_js
    .into_serde()
    .expect("Paramerer should be an OrderedSetItem.");
  ordered_set::public_js::find(id, &item)
}

#[wasm_bindgen]
pub fn ordered_set_slice(id_js: &JsValue, start_js: &JsValue, end_js: &JsValue) -> Vec<u32> {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let start: usize = start_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let end: usize = end_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::slice(id, start, end)
}

#[wasm_bindgen]
pub fn ordered_set_shift(val: &JsValue) -> i64 {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::shift(id)
}

#[wasm_bindgen]
pub fn ordered_set_pop(val: &JsValue) -> i64 {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::pop(id)
}

#[wasm_bindgen]
pub fn ordered_set_remove(id_js: &JsValue, items_js: &JsValue) -> Vec<u32> {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let items: Vec<ordered_set::set_item::OrderedSetItem> = items_js
    .into_serde()
    .expect("Paramerer should be an OrderedSetItem.");
  ordered_set::public_js::remove(id, &items)
}

#[wasm_bindgen]
pub fn ordered_set_remove_at(id_js: &JsValue, index_js: &JsValue) -> i64 {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let index: usize = index_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::remove_at(id, index)
}

#[wasm_bindgen]
pub fn ordered_set_delete(val: &JsValue) -> () {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::delete(id)
}

#[wasm_bindgen]
pub fn ordered_set_clear(val: &JsValue) -> () {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::clear(id)
}

#[wasm_bindgen]
pub fn ordered_set_union(id_js: &JsValue, other_js: &JsValue) -> Vec<u32> {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let other: usize = other_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::union(id, other)
}

#[wasm_bindgen]
pub fn ordered_set_intersection(id_js: &JsValue, other_js: &JsValue) -> Vec<u32> {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let other: usize = other_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::intersection(id, other)
}

#[wasm_bindgen]
pub fn ordered_set_difference(id_js: &JsValue, other_js: &JsValue) -> Vec<u32> {
  let id: usize = id_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  let other: usize = other_js
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::difference(id, other)
}

#[wasm_bindgen]
pub fn ordered_set_size(val: &JsValue) -> usize {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::size(id)
}

#[wasm_bindgen]
pub fn ordered_set_to_array(val: &JsValue) -> Vec<u32> {
  let id: usize = val
    .into_serde()
    .expect("Paramerer should be a positive number.");
  ordered_set::public_js::to_array(id)
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  // Your code goes here!
  console::log_1(&JsValue::from_str("Hello world!"));

  Ok(())
}
