use std::iter::FromIterator;
use std::mem;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Queue<T>(Vec<T>, Vec<T>);

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue(vec![], vec![])
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.move_to_second_stack();
        self.1.pop()
    }

    fn move_to_second_stack(&mut self) {
        let v = mem::take(&mut self.0);
        self.1.extend(v);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_test() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(Queue::new(), queue);
    }

    #[test]
    fn push_test() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(10);

        assert_eq!(queue, Queue(vec![10], vec![]));
    }

    #[test]
    fn push_then_pop() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(10);
        let popped = queue.pop();

        assert_eq!(popped, Some(10));
    }

    #[test]
    fn push_then_pop_loop() {
        let mut queue: Queue<i32> = Queue::new();
        for i in 1..10 {
            queue.push(i);
        }
        for i in (1..10).rev() {
            let popped = queue.pop();
            assert_eq!(popped, Some(i));
        }
    }

    #[test]
    fn from_iter_test() {
        let iter = 1..5;
        let mut queue = Queue::from_iter(iter);

        for i in (1..5).rev() {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn into_iter_test() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        let mut iter = queue.into_iter();
        for i in 1..3 {
            assert_eq!(Some(i), iter.next());
        }
    }
}
