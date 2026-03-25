









pub struct LinearMap[K, V] {
  storage : Array[(K, V)]
}
pub fn LinearMap::new() -> LinearMap {
  { storage: [] }
}
pub fn LinearMap::with_capacity(capacity : Int) -> LinearMap {
  { storage: Array.with_capacity(capacity) }
}
pub fn LinearMap::capacity(self) -> Int {
  self.storage.capacity()
}
pub fn LinearMap::reserve(self, additional : Int) {
  self.storage.reserve(additional)
}
pub fn LinearMap::reserve_exact(self, additional : Int) {
  self.storage.reserve_exact(additional)
}
pub fn LinearMap::shrink_to_fit(self) {
  self.storage.shrink_to_fit()
}
pub fn LinearMap::len(self) -> Int {
  self.storage.length()
}
pub fn LinearMap::is_empty(self) -> Bool {
  self.storage.is_empty()
}
pub fn LinearMap::clear(self) {
  self.storage.clear()
}
pub fn LinearMap::retain(self, keep_fn : F) {
  let mut del = 0
    let v = self.storage
  for i in 0..<v.length() {
    if not(keep_fn(v[i].0, v[i].1)) {
      del += 1
    } else {
      if del > 0 {
        v.swap(i - del, i)
      }
    }
  }
  if del > 0 {
    let len = self.storage.length()
    self.storage.truncate(len - del)
  }
}
pub fn LinearMap::drain(self) -> Drain[K, V] {
  { iter: self.storage.drain(..<) }
}
pub fn LinearMap::iter(self) -> Iter[K, V] {
  { iter: self.storage.iter() }
}
pub fn LinearMap::iter_mut(self) -> IterMut[K, V] {
  { iter: self.storage.iter_mut() }
}
pub fn LinearMap::keys(self) -> Keys[K, V] {
  { iter: self.iter() }
}
pub fn LinearMap::values(self) -> Values[K, V] {
  { iter: self.iter() }
}
pub fn LinearMap::get(self, key : Q) -> Option[V] {
  for (k, v) in self {
    if key == k.borrow() {
      return Some(v)
    }
  }
  None
}
pub fn LinearMap::get_mut(self, key : Q) -> Option[V] {
  for (k, v) in self {
    if key == k.borrow() {
      return Some(v)
    }
  }
  None
}
pub fn LinearMap::contains_key(self, key : Q) -> Bool {
  self.get(key).is_some()
}
pub fn LinearMap::insert(self, key : K, value : V) -> Option[V] {
  match self.entry(key) {
    Occupied(e) => Some(e.insert(value))
    Vacant(e) => {
      e.insert(value)
      None
    }
  }
}
pub fn LinearMap::remove(self, key : Q) -> Option[V] {
  for i in 0..<self.storage.length() {
    if self.storage[i].0.borrow() == key {
      return Some(self.storage.swap_remove(i).1)
    }
  }
  None
}
pub fn LinearMap::entry(self, key : K) -> Entry[K, V] {
  match self.storage.iter().position(fn(_) { key == k }) {
    None => Vacant({ map: self, key: key })
    Some(index) => Occupied({ map: self, index: index })
  }
}

impl Clone for LinearMap[K, V] with clone(self) -> LinearMap {
  { storage: self.storage.copy() }
}
impl Clone for LinearMap[K, V] with clone_from(self, other : LinearMap) {
  self.storage.clone_from(other.storage)
}

impl Debug for LinearMap[K, V] with fmt(self, f : Formatter) -> Result {
  f.debug_map().entries(self).finish()
}

impl Default for LinearMap[K, V] with default(self) -> LinearMap {
  Self.new()
}

impl Extend for LinearMap[K, V] with extend(self, key_values : I) {
  for (key, value) in key_values {
    self.insert(key, value)
  }
}

impl iter.FromIterator for LinearMap[K, V] with from_iter(self, key_values : I) -> LinearMap {
  let mut map = Self.new()
  map.extend(key_values)
  map
}

impl ops.Index for LinearMap[K, V] with index(self, key : Q) -> V {
  self.get(key).expect("key not found")
}

impl PartialEq for LinearMap[K, V] with eq(self, other : LinearMap) -> Bool {
  if self.length() != other.length() {
    return false
  }
  for (key, value) in self {
    if other.get(key) != Some(value) {
      return false
    }
  }
  true
}


impl From for Array[(K, V)] with from(self, other : LinearMap[K, V]) -> Array {
  other.storage
}

impl From for LinearMap[K, V] with from(self, other : Array[(K, V)]) -> LinearMap {
  { storage: other }
}

// TODO(transpile): unsupported item

pub struct OccupiedEntry[K : , V : ] {
  map : LinearMap[K, V]
  index : Int
}
pub struct VacantEntry[K : , V : ] {
  map : LinearMap[K, V]
  key : K
}
pub enum Entry[K : , V : ] {
  Occupied(OccupiedEntry[K, V])
  Vacant(VacantEntry[K, V])
}
pub fn Entry::or_insert(self, default : V) -> V {
  match self {
    Occupied(entry) => entry.into_mut()
    Vacant(entry) => entry.insert(default)
  }
}
pub fn Entry::or_insert_with(self, default : F) -> V {
  match self {
    Occupied(entry) => entry.into_mut()
    Vacant(entry) => entry.insert(default())
  }
}

pub fn OccupiedEntry::get(self) -> V {
  self.map.storage[self.index].1
}
pub fn OccupiedEntry::get_mut(self) -> V {
  self.map.storage[self.index].1
}
pub fn OccupiedEntry::into_mut(self) -> V {
  self.map.storage[self.index].1
}
pub fn OccupiedEntry::insert(self, value : V) -> V {
  mem.replace(self.get_mut(), value)
}
pub fn OccupiedEntry::remove(self) -> V {
  self.map.storage.swap_remove(self.index).1
}

pub fn VacantEntry::insert(self, value : V) -> V {
  self.map.storage.push((self.key, value))
  self.map.storage.last_mut().unwrap().1
}

pub struct IntoIter[K, V] {
  iter : IntoIter[(K, V)]
}
impl Iterator for IntoIter[K, V] with next(self) -> Option[(K, V)] {
  self.iter.next()
}
impl Iterator for IntoIter[K, V] with size_hint(self) -> (Int, Option[Int]) {
  self.iter.size_hint()
}

impl DoubleEndedIterator for IntoIter[K, V] with next_back(self) -> Option[(K, V)] {
  self.iter.next_back()
}

impl ExactSizeIterator for IntoIter[K, V] with len(self) -> Int {
  self.iter.length()
}

pub struct Drain[K : , V : ] {
  iter : Drain[(K, V)]
}
pub struct Iter[K : , V : ] {
  iter : Iter[(K, V)]
}
pub struct IterMut[K : , V : ] {
  iter : IterMut[(K, V)]
}
pub struct Keys[K : , V : ] {
  iter : Iter[K, V]
}
pub struct Values[K : , V : ] {
  iter : Iter[K, V]
}
// TODO(transpile): unsupported item

// TODO(transpile): unsupported item

// TODO(transpile): unsupported item

// TODO(transpile): unsupported item

// TODO(transpile): unsupported item

// TODO(transpile): unsupported item

impl Clone for Iter[K, V] with clone(self) -> Iter {
  { iter: self.iter.copy() }
}

impl Clone for Keys[K, V] with clone(self) -> Keys {
  { iter: self.iter.copy() }
}

impl Clone for Values[K, V] with clone(self) -> Values {
  { iter: self.iter.copy() }
}

impl IntoIterator for LinearMap[K, V] with into_iter(self) -> IntoIter[K, V] {
  { iter: self.storage.into_iter() }
}

impl IntoIterator for LinearMap[K, V] with into_iter(self) -> Iter[K, V] {
  self.iter()
}

impl IntoIterator for LinearMap[K, V] with into_iter(self) -> IterMut[K, V] {
  self.iter_mut()
}

fn assert_covariance() {
    fn[K, V] a(x : LinearMap[K, V]) -> LinearMap[K, V] {
    x
  }
    fn[K, V] b(x : IntoIter[K, V]) -> IntoIter[K, V] {
    x
  }
    fn[K, V] c(x : Iter[K, V]) -> Iter[K, V] {
    x
  }
    fn[K, V] d(x : Keys[K, V]) -> Keys[K, V] {
    x
  }
    fn[K, V] e(x : Values[K, V]) -> Values[K, V] {
    x
  }
}
