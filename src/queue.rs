use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct SimpleQueue {
    data: Vec<i32>,
    head: usize,
}

impl SimpleQueue {
    pub fn new() -> SimpleQueue {
        SimpleQueue {
            data: Vec::new(),
            head: 0,
        }
    }

    pub fn enqueue(&mut self, v: i32) -> bool {
        self.data.push(v);
        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        match self.data.get(self.head) {
            None => None,
            Some(v) => {
                self.head += 1;
                Some(*v)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.head
    }
}

#[cfg(test)]
mod tests_simple_queue {
    use crate::SimpleQueue;

    #[test]
    fn test_new() {
        let q = SimpleQueue::new();
        assert_eq!(q.data, vec![]);
        assert_eq!(q.head, 0);
    }

    #[test]
    fn test_enqueue() {
        let mut q = SimpleQueue::new();
        for i in 1..=10 {
            q.enqueue(i);
        }
        assert_eq!(q.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    }

    #[test]
    fn test_dequeue() {
        let mut q = SimpleQueue::new();
        assert_eq!(q.dequeue(), None);

        for i in 1..=10 {
            q.enqueue(i);
            assert_eq!(q.len(), i as usize);
        }

        for i in 1..=10 {
            assert_eq!(q.dequeue(), Some(i));
            assert_eq!(q.len(), (10 - i) as usize);
        }

        assert_eq!(q.dequeue(), None);
    }
}

#[derive(Debug)]
pub struct FixedCapacitySimpleQueue {
    data: Vec<i32>,
    capacity: usize,
    head: usize,
}

impl FixedCapacitySimpleQueue {
    pub fn new(capacity: usize) -> FixedCapacitySimpleQueue {
        FixedCapacitySimpleQueue {
            data: Vec::new(),
            capacity,
            head: 0,
        }
    }

    pub fn enqueue(&mut self, v: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.data.push(v);
        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let v = *self.data.get(self.head).unwrap();
        self.head += 1;
        Some(v)
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.head >= self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.head
    }
}

#[cfg(test)]
mod tests_simple_fixed_capacity_queue {
    use crate::FixedCapacitySimpleQueue;

    #[test]
    fn test_new() {
        let q = FixedCapacitySimpleQueue::new(16);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.capacity, 16);
    }

    #[test]
    fn test_enqueue() {
        let mut q = FixedCapacitySimpleQueue::new(16);
        for i in 1..=16 {
            assert!(q.enqueue(i));
            assert_eq!(q.len(), i as usize);
        }
        assert!(!q.enqueue(17));
        assert_eq!(q.len(), 16);
    }

    #[test]
    fn test_dequeue() {
        let mut q = FixedCapacitySimpleQueue::new(16);
        assert_eq!(q.dequeue(), None);

        for i in 1..=16 {
            assert!(q.enqueue(i));
            assert_eq!(q.len(), i as usize);
        }
        assert!(!q.enqueue(17));

        for i in 1..=16 {
            assert_eq!(q.dequeue(), Some(i));
            assert_eq!(q.len(), (16 - i) as usize);
        }
        assert_eq!(q.dequeue(), None);
        assert_eq!(q.len(), 0);
    }
}

#[derive(Debug)]
pub struct FixedCapacityRingQueue {
    data: Vec<i32>,
    capacity: usize,
    head: usize,
    tail: usize,
    cross_circle: bool,
}

impl FixedCapacityRingQueue {
    pub fn new(capacity: usize) -> FixedCapacityRingQueue {
        FixedCapacityRingQueue {
            data: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            tail: 0,
            cross_circle: false,
        }
    }

    pub fn enqueue(&mut self, v: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.data.len() < self.data.capacity() {
            self.data.push(v);
        } else {
            let res = self.data.index_mut(self.tail);
            *res = v;
        }
        self.move_tail();

        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let v = *self.data.get(self.head).unwrap();
        self.move_head();

        Some(v)
    }

    pub fn is_empty(&self) -> bool {
        !self.cross_circle && self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.cross_circle && self.head == self.tail
    }

    pub fn len(&self) -> usize {
        match self.cross_circle {
            false => self.tail - self.head,
            true => self.tail + self.capacity - self.head,
        }
    }

    // return the first element of queue
    pub fn front(&self) -> Option<i32> {
        self.read_index(self.head)
    }

    // return the last element of queue
    pub fn rear(&self) -> Option<i32> {
        self.read_index((self.tail + self.capacity - 1) % self.capacity)
    }

    fn read_index(&self, i: usize) -> Option<i32> {
        if self.is_empty() || i >= self.capacity {
            return None;
        }

        let v = *self.data.get(i).unwrap();
        Some(v)
    }

    fn move_head(&mut self) {
        let increase_head = self.head + 1;
        self.head = increase_head % self.capacity;
        if increase_head >= self.capacity {
            self.cross_circle = false;
        }
    }

    fn move_tail(&mut self) {
        let increase_tail = self.tail + 1;
        self.tail = increase_tail % self.capacity;
        if increase_tail >= self.capacity {
            self.cross_circle = true;
        }
    }
}

#[cfg(test)]
mod tests_fixed_capacity_ring_queue {
    use crate::FixedCapacityRingQueue;

    #[test]
    fn test_new() {
        let q = FixedCapacityRingQueue::new(16);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn test_enqueue() {
        let mut q = FixedCapacityRingQueue::new(16);
        assert_eq!(q.front(), None);
        assert_eq!(q.rear(), None);

        for i in 1..=16 {
            assert!(q.enqueue(i));
        }
        assert_eq!(q.len(), 16);
        assert!(!q.enqueue(17));
        assert_eq!(q.front(), Some(1));
        assert_eq!(q.rear(), Some(16));

        assert_eq!(
            q.data,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        );
    }

    #[test]
    fn test_dequeue() {
        let mut q = FixedCapacityRingQueue::new(16);
        for i in 1..=16 {
            assert!(q.enqueue(i));
        }
        assert!(q.is_full());

        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert!(!q.is_full());
        assert_eq!(q.front(), Some(4));

        for i in 17..=19 {
            assert!(q.enqueue(i));
        }
        assert!(q.is_full());
        assert!(!q.enqueue(20));
        assert_eq!(q.rear(), Some(19));

        assert_eq!(
            q.data,
            vec![17, 18, 19, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        );

        for _ in 0..q.len() {
            q.dequeue();
        }
        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());
    }
}

mod tests_rust_vec_queue {
    use std::collections::VecDeque;

    #[test]
    fn test() {
        let mut q = VecDeque::with_capacity(16);
        assert!(q.is_empty());

        for i in 1..=16 {
            q.push_back(i);
        }
        assert_eq!(
            q,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        );

        for i in 1..=16 {
            assert_eq!(q.pop_front(), Some(i));
        }
        assert_eq!(q.pop_front(), None);
        assert_eq!(q.len(), 0);
    }
}
