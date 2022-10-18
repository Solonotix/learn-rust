use std::collections::hash_map::{Keys,Values};
use std::slice::Iter;

pub trait ExistsWithin<T> : Iterator {
    fn exists_within<P>(self, predicate: P) -> bool where P: FnMut(Self::Item) -> bool;
}

impl <T> ExistsWithin<T> for Iter<'_, T> {
    fn exists_within<P>(self, mut predicate: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self {
            if predicate(item) {
                return true;
            }
        }

        return false;
    }
}

impl <K, V> ExistsWithin<V> for Keys<'_, K, V> {
    fn exists_within<P>(self, mut predicate: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self {
            if predicate(item) {
                return true;
            }
        }

        return false;
    }
}

impl <K, V> ExistsWithin<V> for Values<'_, K, V> {
    fn exists_within<P>(self, mut predicate: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self {
            if predicate(item) {
                return true;
            }
        }

        return false;
    }
}