pub(crate) struct Map<K, V>(Vec<(K, V)>);

impl<K, V> Map<K, V>
where
  K: PartialEq,
{
  pub(crate) const fn new() -> Self {
    Self(Vec::new())
  }

  pub(crate) fn get(&self, key: &K) -> Option<&V> {
    self.0.iter().find_map(|(k, v)| (k == key).then_some(v))
  }

  pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
    let old = self
      .0
      .iter()
      .position(|(k, _)| k == &key)
      .map(|i| self.0.remove(i).1);
    self.0.push((key, value));
    old
  }

  pub(crate) fn retain(&mut self, f: impl FnMut(&(K, V)) -> bool) {
    self.0.retain(f);
  }
}
