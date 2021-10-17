#![deny(missing_docs)]

use std::iter::FromIterator;
use std::mem;

#[derive(Default, Debug, Eq, PartialEq)]
/// A queue created with two stacks.
pub struct Queue<T>(Vec<T>, Vec<T>);

impl<T> Queue<T> {
    /// Creates a new Queue.
    pub fn new() -> Self {
        Queue(vec![], vec![])
    }

    /// Adds an item to the end of the queue in O(1) time.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Removes the first item from the queue in O(n) time.
    pub fn pop(&mut self) -> Option<T> {
        self.move_to_second_stack();
        self.1.pop()
    }

    fn move_to_second_stack(&mut self) {
        let v = mem::take(&mut self.0);
        let iter = v.into_iter().rev();
        self.1.extend(iter);
    }
}

impl<T> FromIterator<T> for Queue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Queue<T> {
        let mut queue = Queue::new();
        for i in iter {
            queue.push(i);
        }
        queue
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<T>, std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().chain(self.1.into_iter())
    }
}

/// Create a new `Queue` with the elements inside the macro.
/// Works like the `vec![]` macro.
/// ## Examples
/// ```
/// # use data_structures_in_rust::queue;
/// # use data_structures_in_rust::queue_with_stack::*;
/// let queue = queue![1, 2, 3];
/// let empty: Queue<i32> = queue![];
/// let mut other = Queue::new();
/// other.push(1);
/// other.push(2);
/// other.push(3);
/// assert_eq!(queue, other);
/// assert_eq!(empty, Queue::new());
/// ```
#[macro_export]
macro_rules! queue [
    ($($e:expr),*) => ({
        let mut _temp  = Queue::new();
        $(_temp.push($e);)*
        _temp
    })
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(Queue::new(), queue);
    }

    #[test]
    fn push_test() {
        let mut queue = queue![];
        queue.push(10);

        assert_eq!(queue, Queue(vec![10], vec![]));
    }

    #[test]
    fn pop_test() {
        let mut queue = queue![10];

        assert_eq!(queue.pop(), Some(10));
    }

    #[test]
    fn push_then_pop_loop() {
        let mut queue = queue![];
        for i in 1..10 {
            queue.push(i);
        }
        for i in 1..10 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn from_iter_test() {
        let iter = 1..5;
        let mut queue = Queue::from_iter(iter);

        for i in 1..5 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn into_iter_test() {
        let queue = queue![1, 2, 3];

        let mut iter = queue.into_iter();
        for i in 1..3 {
            assert_eq!(Some(i), iter.next());
        }
    }
}
