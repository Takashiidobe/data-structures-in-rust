use std::cmp::min;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut, Index};

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Item<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    curr: T,
    min: T,
}

impl<T> Item<T>
where
    T: Ord,
{
    fn new(curr: T, min: T) -> Self {
        Item { curr, min }
    }
}

impl<T> MinStack<T>
where
    T: Ord + Default + Clone,
{
    pub fn append(&mut self, other: &mut Self) {
        let other = mem::take(other);
        for item in other {
            self.push(item.curr);
        }
    }
}

impl<T> Default for MinStack<T>
where
    T: Ord,
{
    fn default() -> MinStack<T> {
        MinStack::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MinStack<T: Ord>(Vec<Item<T>>);

impl<T> IntoIterator for MinStack<T>
where
    T: Ord,
{
    type Item = Item<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Deref for MinStack<T>
where
    T: Ord,
{
    type Target = [Item<T>];
    fn deref(&self) -> &[Item<T>] {
        self.0.deref()
    }
}

impl<T> DerefMut for MinStack<T>
where
    T: Ord,
{
    fn deref_mut(&mut self) -> &mut [Item<T>] {
        self.0.deref_mut()
    }
}

impl<T> From<Vec<T>> for MinStack<T>
where
    T: Clone + Ord,
{
    fn from(vec: Vec<T>) -> Self {
        let mut stack = MinStack::new();
        for item in vec {
            stack.push(item);
        }
        stack
    }
}

impl<T> Index<usize> for MinStack<T>
where
    T: Ord,
{
    type Output = Item<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> MinStack<T>
where
    T: Clone + Ord,
{
    pub fn push(&mut self, item: T) {
        if !self.0.is_empty() {
            let curr_min = min(self.peek_min().unwrap(), item.clone());
            self.0.push(Item::new(item, curr_min));
        } else {
            self.0.push(Item::new(item.clone(), item));
        }
    }

    pub fn min(&self) -> Option<T> {
        self.0.last().map(|item| item.clone().min)
    }

    pub fn peek(&self) -> Option<T> {
        self.0.last().map(|item| item.clone().curr)
    }

    pub fn peek_min(&self) -> Option<T> {
        self.0.last().map(|item| item.clone().min)
    }
}

impl<T> MinStack<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        MinStack(vec![])
    }

    pub fn as_slice(&self) -> &[Item<T>] {
        &self.0
    }

    pub fn as_ptr(&self) -> *const Item<T> {
        self.0.as_ptr()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn with_capacity(capacity: usize) -> MinStack<T> {
        let mut stack = MinStack::new();
        stack.0 = Vec::with_capacity(capacity);
        stack
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(item) => Some(item.curr),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[macro_export]
macro_rules! min_stack [
    ($($e:expr),*) => ({
        let mut _temp  = MinStack::new();
        $(_temp.push($e);)*
        _temp
    })
];

#[cfg(test)]
mod tests {
    use super::{Item, MinStack};

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

    #[test]
    fn append_empty_stack() {
        let mut left = min_stack![1];
        let mut right = min_stack![];
        left.append(&mut right);
        assert_eq!(left, min_stack![1]);
        assert_eq!(right, min_stack![]);
    }

    #[test]
    fn append_to_empty_stack() {
        let mut left = min_stack![];
        let mut right = min_stack![1];
        left.append(&mut right);
        assert_eq!(left, min_stack![1]);
        assert_eq!(right, min_stack![]);
    }

    #[test]
    fn append_two_empty_stacks() {
        let mut left: MinStack<Item<u32>> = min_stack![];
        let mut right = min_stack![];
        left.append(&mut right);
        assert_eq!(left, min_stack![]);
        assert_eq!(right, min_stack![]);
    }

    #[test]
    fn append_two_stacks() {
        let mut left = min_stack![1];
        let mut right = min_stack![2];
        left.append(&mut right);
        assert_eq!(left, min_stack![1, 2]);
        assert_eq!(right, min_stack![]);
    }

    #[test]
    fn test_eq_stacks_1() {
        let left = min_stack![1, 2];
        let right = min_stack![1, 2];
        assert_eq!(left, right);
    }

    #[test]
    fn test_neq_stacks_1() {
        let left = min_stack![2, 1];
        let right = min_stack![1, 2];
        assert_ne!(left, right);
    }
}
