#![deny(missing_docs)]

use std::cmp::min;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut, Index};

impl<T> MinStack<T>
where
    T: Clone + Ord + Default,
{
    /// Moves all the elements of `other` into `Self`, leaving other empty.
    /// ## Panics
    /// Panics if the number of elements in the `MinStack` overflows a `usize`.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::MinStack;
    /// let mut left = min_stack![1];
    /// let mut right = min_stack![2];
    /// left.append(&mut right);
    /// assert_eq!(left, min_stack![1, 2]);
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        let other = mem::take(other);
        for item in other {
            self.push(item.0);
        }
    }
}

impl<T> MinStack<T>
where
    T: Clone + Ord,
{
    /// Finds the minimum item of the stack in O(1) time.
    /// If the stack is empty, returns `None`.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::MinStack;
    /// let stack = min_stack![1, 2];
    /// let empty: MinStack<i32> = min_stack![];
    /// assert_eq!(stack.min(), Some(1));
    /// assert_eq!(empty.min(), None);
    /// ```
    pub fn min(&self) -> Option<T> {
        self.0.last().map(|item| item.1.clone())
    }

    /// Look at the top item of the stack in O(1) time.
    /// If the stack is empty, returns `None`.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::MinStack;
    /// let stack = min_stack![1, 2];
    /// let empty: MinStack<i32> = min_stack![];
    /// assert_eq!(stack.peek(), Some(2));
    /// assert_eq!(empty.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<T> {
        self.0.last().map(|item| item.0.clone())
    }

    /// Adds an item to the end of the stack in O(1) time.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::MinStack;
    /// let mut stack = min_stack![1];
    /// stack.push(2);
    /// assert_eq!(stack.len(), 2);
    /// ```
    pub fn push(&mut self, item: T) {
        if !self.0.is_empty() {
            let curr_min = self.min().clone().unwrap();
            let new_min = min(curr_min, item.clone());
            self.0.push((item, new_min));
        } else {
            self.0.push((item.clone(), item));
        }
    }

    /// Creates a min_stack from a vector.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::MinStack;
    /// let v = vec![1, 2, 3];
    /// let stack: MinStack<i32> = MinStack::from(v);
    /// assert_eq!(stack, min_stack![1, 2, 3]);
    /// ```
    pub fn from(vec: Vec<T>) -> Self {
        let mut stack = MinStack::new();
        for item in vec {
            stack.push(item);
        }
        stack
    }
}

/// A Stack data type that supports accessing the minimum item
/// in the stack in O(1) time.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MinStack<T: Ord>(Vec<(T, T)>);

impl<T> IntoIterator for MinStack<T>
where
    T: Ord,
{
    type Item = (T, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Deref for MinStack<T>
where
    T: Ord,
{
    type Target = [(T, T)];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> DerefMut for MinStack<T>
where
    T: Ord,
{
    fn deref_mut(&mut self) -> &mut [(T, T)] {
        self.0.deref_mut()
    }
}

impl<T> Index<usize> for MinStack<T>
where
    T: Ord,
{
    type Output = (T, T);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> MinStack<T>
where
    T: Ord,
{
    /// Creates a new MinStack.
    pub fn new() -> Self {
        MinStack(vec![])
    }

    /// Extracts a slice containing the Min Stack.
    /// Equivalent to `&s[..]`.
    pub fn as_slice(&self) -> &[(T, T)] {
        &self.0
    }

    /// Returns a raw pointer to the min_stack.
    pub fn as_ptr(&self) -> *const (T, T) {
        self.0.as_ptr()
    }

    /// Returns a mutable pointer to the min_stack.
    pub fn as_mut_ptr(&mut self) -> *mut (T, T) {
        self.0.as_mut_ptr()
    }

    /// Clears the MinStack, removing all values.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let mut stack = min_stack![1,2,3];
    /// stack.clear();
    /// assert_eq!(stack, min_stack![]);
    /// ```
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Creates a new MinStack with the given capacity.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let stack: MinStack<i32> = MinStack::with_capacity(10);
    /// assert!(stack.capacity() >= 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> MinStack<T> {
        let mut stack = MinStack::new();
        stack.0 = Vec::with_capacity(capacity);
        stack
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `MinStack<T>`. The collection may reserve more space to
    /// avoid frequent reallocations. After calling reserve, capacity will be
    /// greater than or equal to self.len() + additional.
    /// Does nothing if capacity is already sufficient.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let mut stack = min_stack![1];
    /// stack.reserve(10);
    /// assert!(stack.capacity() >= 11);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Returns the number of elements the vector can hold without reallocating.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let stack: MinStack<i32> = MinStack::with_capacity(10);
    /// assert!(stack.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Removes the last element from a vector and returns it, or `None` if it is empty.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let mut stack = min_stack![1, 2, 3];
    /// let mut empty: MinStack<i32> = min_stack![];
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(empty.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(item) => Some(item.0),
            None => None,
        }
    }

    /// Returns `true` if the MinStack has no elements.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let stack = min_stack![1, 2, 3];
    /// let empty: MinStack<i32> = min_stack![];
    /// assert_eq!(stack.is_empty(), false);
    /// assert_eq!(empty.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of elements in the stack.
    /// ## Examples
    /// ```
    /// # use data_structures_in_rust::min_stack;
    /// # use data_structures_in_rust::min_stack::*;
    /// let stack = min_stack![1, 2, 3];
    /// assert_eq!(stack.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Create a new min_stack with the elements inside the macro.
/// Works like the `vec![]` macro.
/// ## Examples
/// ```
/// # use data_structures_in_rust::min_stack;
/// # use data_structures_in_rust::min_stack::*;
/// let stack = min_stack![1, 2, 3];
/// let empty: MinStack<i32> = min_stack![];
/// let mut other = MinStack::new();
/// other.push(1);
/// other.push(2);
/// other.push(3);
/// assert_eq!(stack, other);
/// assert_eq!(empty, MinStack::new());
/// ```
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
    use super::MinStack;

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
        let empty: MinStack<i32> = min_stack![1];
        assert_eq!(empty.is_empty(), false);
    }

    #[test]
    fn empty_len_test() {
        let empty: MinStack<i32> = min_stack![];
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn one_item_stack_len() {
        let empty: MinStack<i32> = min_stack![1];
        assert_eq!(empty.len(), 1);
    }

    #[test]
    fn into_iter_test_1() {
        let mut stack = min_stack![1, 2, 3].into_iter();
        assert_eq!(stack.next(), Some((1, 1)));
        assert_eq!(stack.next(), Some((2, 1)));
        assert_eq!(stack.next(), Some((3, 1)));
    }

    #[test]
    fn from_vec_test_1() {
        let vec = vec![1, 2, 3];
        assert_eq!(MinStack::from(vec), min_stack![1, 2, 3]);
    }

    #[test]
    fn test_index_1() {
        let stack = min_stack![1];
        assert_eq!(stack[0], (1, 1));
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
        assert_eq!([(1, 1), (2, 1), (3, 1)], stack.as_slice());
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
        let mut left: MinStack<i32> = min_stack![];
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
