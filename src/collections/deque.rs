use alloc::vec_deque::VecDeque;
#[cfg(test)]
use alloc::vec_deque::Iter;

/// Double ended Queue with fixed size
pub struct Deque<T> {
    capacity: usize,
    stack: VecDeque<T>,
}
impl<T> Deque<T> {
    pub fn new(max_size: usize) -> Deque<T> {
        if max_size == 0 {
            panic!("deque size capacity should be >1")
        }

        let stack = VecDeque::with_capacity(max_size);
        Deque {
            capacity: max_size,
            stack,
        }
    }
    #[cfg(test)]
    pub fn values(&self) -> &VecDeque<T> {
        &self.stack
    }
    pub fn push(&mut self, value: T) {
        if self.stack.len() == self.capacity {
            self.pop();
        }
        self.stack.push_back(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop_front()
    }
    #[cfg(test)]
    pub fn iter(&self) -> Iter<T> {
        self.stack.iter()
    }
}

#[cfg(test)]
pub mod tests {
    extern crate rand;
    use self::rand::thread_rng;
    use self::rand::distributions::{IndependentSample, Range};

    use super::*;

    #[test]
    fn is_empty() {
        let d: Deque<u8> = Deque::new(rand_size());
        assert_eq!(d.values().len(), 0);
    }
    #[test]
    #[should_panic]
    fn no_null_capacity() {
        let _: Deque<u8> = Deque::new(0);
    }
    #[test]
    fn max_size() {
        let s = rand_size();
        let mut deque = Deque::new(s);

        for _ in 0..2 * s {
            deque.push(0);
        }
        assert_eq!(deque.values().len(), s);
    }
    #[test]
    fn insert() {
        let mut deque = Deque::new(3);

        let data = [1, 2, 3];

        for d in data.iter() {
            deque.push(d.clone());
        }

        assert_eq!(deque.values().len(), data.len());
        for (i, &d) in deque.iter().enumerate() {
            assert_eq!(d, data[i]);
        }

        let data = [4, 5, 6];
        for d in data.iter() {
            deque.push(d.clone());
        }

        assert_eq!(deque.values().len(), data.len());
        for (i, &d) in deque.iter().enumerate() {
            assert_eq!(d, data[i]);
        }
    }
    #[test]
    fn push_and_pop() {
        let mut deque = Deque::new(2);

        deque.push(1);
        deque.push(2);
        deque.push(3);
        assert_eq!(deque.pop(), Some(2));
        assert_eq!(deque.pop(), Some(3));

    }
    fn rand_size() -> usize {
        let mut rng = thread_rng();
        Range::new(1, 42).ind_sample(&mut rng)
    }
}
