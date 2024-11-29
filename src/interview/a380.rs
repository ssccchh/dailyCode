use rand::Rng;
use std::collections::HashMap;

use crate::Solution;

struct RandomizedSet {
    set: Vec<i32>,
    cache: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: vec![],
            cache: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.cache.contains_key(&val) {
            self.set.push(val);
            self.cache.insert(val, self.set.len() - 1);
            return true;
        } else {
            return false;
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.cache.get(&val) {
            Some(&index) => {
                // 要求是O(1)，所以用最后一项覆盖要删除项代替直接删除
                let last = self.set[self.set.len() - 1];
                self.set[index] = last;
                self.cache.insert(last, index);
                self.cache.remove(&val);
                self.set.pop();
                println!("index: {}, val: {}", index, val);
                return true;
            }
            None => return false,
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..=(self.set.len() - 1));
        self.set[idx]
    }
}

impl Solution {
    pub fn run() {
        let mut random = RandomizedSet::new();
        let r1 = random.insert(0);
        let r2 = random.insert(1);
        let r3 = random.remove(0);
        let r4 = random.insert(2);
        let r5 = random.remove(1);
        let r6 = random.get_random();

        println!("{:?}, random: {}", vec![r1, r2, r3, r4, r5], r6);
        println!("{:?}", random.set);
    }
}
