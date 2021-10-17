use std::collections::VecDeque;

pub struct Stack<T>(VecDeque<T>, VecDeque<T>);

// use two queues for a stack.
// enqueue to the first stack, dequeue off the second.
