use slotmap::Key;
use slotmap::basic::IterMut;
use slotmap::basic::IntoIter;
use slotmap::basic::Iter;

use crate::Tree;

/// # Purpose:
/// Iterator methods.
impl<K, V> Tree<K, V>
where
    K: Key,
{
    /// # Purpose:
    /// Create an immutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.values.iter()
    }

    /// # Purpose:
    /// Create a mutable iterator over the key-value pairs inside of this
    /// [`Tree`] instance.
    ///
    /// Note that this iterator will yield elements of type `(K, &mut V)`.
    /// Namely, this function only provides mutable access to the values, not
    /// the keys!
    ///
    /// The order of iteration is arbitrary. It will not be guaranteed to be
    /// depth-first, breadth-first, in-order, etc.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.values.iter_mut()
    }
}

/// # Purpose:
/// Owned iterator over the key-value pairs in this [`Tree`] instance.
impl<K, V> IntoIterator for Tree<K, V>
where
    K: Key,
{
    type IntoIter = IntoIter<K, V>;
    type Item = <Self::IntoIter as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
