/// A set backed by a `Vec`, preserving insertion order.
///
/// Suitable for small collections where the item count doesn't justify
/// the overhead of a hash set. Iterates in insertion order.
#[derive(Debug, Clone)]
pub struct VecSet<T> {
    items: Vec<T>,
}

impl<T> Default for VecSet<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: PartialEq> VecSet<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a value. Returns `true` if the value was newly inserted,
    /// `false` if it was already present.
    pub fn insert(&mut self, value: T) -> bool {
        if self.items.contains(&value) {
            false
        } else {
            self.items.push(value);
            true
        }
    }

    /// Removes a value. Returns `true` if the value was present.
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item == value) {
            self.items.swap_remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.items.contains(value)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
        self.items.retain(f);
    }
}

impl<T> IntoIterator for VecSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a VecSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
