use std::cmp::min;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index};
use std::slice::SliceIndex;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Item<T>
where
    T: Clone + PartialEq + Eq + PartialOrd + Ord,
{
    curr: T,
    min: T,
}

impl<T> Item<T>
where
    T: Clone + Ord,
{
    fn new(curr: T, min: T) -> Self {
        Item { curr, min }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MinStack<T: Ord + Clone>(Vec<Item<T>>);

impl<T: Clone + Ord> IntoIterator for MinStack<T> {
    type Item = Item<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Clone + Ord> Deref for MinStack<T> {
    type Target = [Item<T>];
    fn deref(&self) -> &[Item<T>] {
        self.0.deref()
    }
}

impl<T: Clone + Ord> DerefMut for MinStack<T> {
    fn deref_mut(&mut self) -> &mut [Item<T>] {
        self.0.deref_mut()
    }
}

impl<T: Clone + Ord> From<Vec<T>> for MinStack<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut stack = MinStack::new();
        for item in vec {
            stack.push(item);
        }
        stack
    }
}

impl<T: Clone + Ord> Index<usize> for MinStack<T> {
    type Output = Item<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Ord + Clone> MinStack<T> {
    fn new() -> Self {
        MinStack(vec![])
    }

    fn as_slice(&self) -> &[Item<T>] {
        &self.0
    }

    fn as_ptr(&self) -> *const Item<T> {
        let ptr = self.0.as_ptr();
        ptr
    }

    // fn append(&mut self, other: &mut Self) {
    //     for item in other.into_iter() {
    //         self.push(item.curr);
    //     }
    // }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn with_capacity(capacity: usize) -> MinStack<T> {
        let mut stack = MinStack::new();
        stack.0 = Vec::with_capacity(capacity);
        stack
    }

    fn push(&mut self, item: T) {
        if !self.0.is_empty() {
            let curr_min = min(self.min().unwrap(), item.clone());
            self.0.push(Item::new(item, curr_min));
        } else {
            self.0.push(Item::new(item.clone(), item.clone()));
        }
    }

    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn peek(&self) -> Option<T> {
        if let Some(curr) = self.0.last() {
            return Some(curr.curr.clone());
        }
        None
    }

    fn peek_min(&self) -> Option<T> {
        if let Some(curr) = self.0.last() {
            return Some(curr.min.clone());
        }
        None
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.0.pop() {
            return Some(item.curr.clone());
        }
        None
    }

    fn min(&self) -> Option<T> {
        if let Some(item) = self.0.last() {
            return Some(item.min.clone());
        }
        None
    }

    // fn get<I>(&self, index: I) -> Option<&I::Output>
    // where
    //     I: SliceIndex<Self>,
    // {
    //     self.0.get(self)
    // }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

macro_rules! min_stack [
    ($($e:expr),*) => ({
        let mut _temp  = MinStack::new();
        $(_temp.push($e);)*
        _temp
    })
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_test_1() {
        let min = min_stack![1, 2, 3].min();
        assert_eq!(min, Some(1));
    }

    #[test]
    fn min_test_empty() {
        let empty: MinStack<i32> = min_stack![];
        assert_eq!(empty.min(), None);
    }

    #[test]
    fn peek_test_1() {
        let stack = min_stack![1, 2, 3].peek();
        assert_eq!(stack, Some(3));
    }

    #[test]
    fn peek_test_empty() {
        let empty: MinStack<i32> = min_stack![];
        assert_eq!(empty.peek(), None);
    }

    #[test]
    fn empty_test() {
        let empty: MinStack<i32> = min_stack![];
        assert_eq!(empty.is_empty(), true);
    }

    #[test]
    fn non_empty_test() {
        let empty = min_stack![1];
        assert_eq!(empty.is_empty(), false);
    }

    #[test]
    fn empty_len_test() {
        let empty: MinStack<i32> = min_stack![];
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn one_item_stack_len() {
        let empty = min_stack![1];
        assert_eq!(empty.len(), 1);
    }

    #[test]
    fn into_iter_test_1() {
        let mut stack = min_stack![1, 2, 3].into_iter();
        assert_eq!(stack.next(), Some(Item::new(1, 1)));
        assert_eq!(stack.next(), Some(Item::new(2, 1)));
        assert_eq!(stack.next(), Some(Item::new(3, 1)));
    }

    #[test]
    fn from_vec_test_1() {
        let vec = vec![1, 2, 3];
        assert_eq!(MinStack::from(vec), min_stack![1, 2, 3]);
    }

    #[test]
    fn test_index_1() {
        let stack = min_stack![1];
        assert_eq!(stack[0], Item::new(1, 1));
    }

    #[test]
    fn test_reserve_1() {
        let mut stack = min_stack![1];
        stack.reserve(10);
        assert!(stack.capacity() >= 11);
    }

    #[test]
    fn test_as_slice_1() {
        let stack = min_stack![1, 2, 3];
        assert_eq!(
            [Item::new(1, 1), Item::new(2, 1), Item::new(3, 1)],
            stack.as_slice()
        );
    }
}
