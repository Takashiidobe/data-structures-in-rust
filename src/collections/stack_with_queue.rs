#![deny(missing_docs)]
use std::collections::VecDeque;

/// A Stack implemented with two queues.
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stack<T>(VecDeque<T>, VecDeque<T>);

impl<T> Stack<T> {
    /// Creates a new Stack.
    pub fn new() -> Stack<T> {
        Stack(VecDeque::default(), VecDeque::default())
    }
    /// Add an item to the top of the stack in O(1) time.
    pub fn push(&mut self, item: T) {
        self.0.push_back(item);
    }
    /// Remove and return the top item of the stack in O(1) time.
    pub fn pop(&mut self) -> Option<T> {
        // pop all the items from the first queue
        // push to the second queue
        // pop from the second queue.
        self.move_to_second();
        self.1.pop_front()
    }

    fn move_to_second(&mut self) {
        let mut temp = VecDeque::new();
        while let Some(item) = self.0.pop_front() {
            temp.push_back(item);
        }
        self.1 = temp.into_iter().rev().collect();
    }
}

/// Create a new min_stack with the elements inside the macro.
/// Works like the `vec![]` macro.
/// ## Examples
/// ```
/// # use stdlib_rs::stack;
/// # use stdlib_rs::collections::stack_with_queue::*;
/// let stack = stack![1, 2, 3];
/// let empty: Stack<i32> = stack![];
/// let mut other = Stack::new();
/// other.push(1);
/// other.push(2);
/// other.push(3);
/// assert_eq!(stack, other);
/// assert_eq!(empty, Stack::new());
/// ```
#[macro_export]
macro_rules! stack [
    ($($e:expr),*) => ({
        let mut _temp  = Stack::default();
        $(_temp.push($e);)*
        _temp
    })
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_test() {
        let mut stack = stack![];
        stack.push(1);
        assert_eq!(stack, stack![1]);
    }

    #[test]
    fn pop_test() {
        let mut stack = stack![1, 2, 3];
        assert_eq!(stack.pop(), Some(3));
    }
}
