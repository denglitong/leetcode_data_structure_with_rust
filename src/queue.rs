use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct SimpleQueue {
    data: Vec<i32>,
    head: usize,
}

pub struct FixedCapacitySimpleQueue {
    data: Vec<i32>,
    capacity: usize,
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

impl FixedCapacitySimpleQueue {
    pub fn new(size: usize) -> FixedCapacitySimpleQueue {
        FixedCapacitySimpleQueue {
            data: Vec::new(),
            capacity: size,
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
