use crate::Solution;

struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![(0, i32::MAX)],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push((val, val.min(self.get_min())))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

impl Solution {
    pub fn run155() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(2);
        stack.pop();
        let _ = stack.top();
        let _ = stack.get_min();
    }
}
