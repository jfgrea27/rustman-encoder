pub mod heap {

    pub enum HeapType {
        Min,
        Max,
    }

    pub enum ChildSide {
        Left,
        Right,
    }

    enum PercolateDirection {
        Up,
        Down,
    }

    pub struct BinaryHeap<T> {
        pub heap_type: HeapType,
        pub heap: Vec<T>, //TODO
    }

    pub fn parent(i: usize) -> Option<usize> {
        if i <= 0 {
            // no parent
            None
        } else if i % 2 == 1 {
            // left side
            Some(i / 2)
        } else {
            // right side
            Some(i / 2 - 1)
        }
    }

    pub fn child(i: usize, side: ChildSide) -> Option<usize> {
        match side {
            ChildSide::Left => {
                if i == 0 {
                    Some(1)
                } else {
                    Some((i * 2) + 1)
                }
            }
            ChildSide::Right => {
                if i == 0 {
                    Some(2)
                } else {
                    Some((i * 2) + 2)
                }
            }
        }
    }

    impl<T> BinaryHeap<T>
    where
        T: std::cmp::PartialOrd + Clone,
    {
        pub fn default() -> Self {
            BinaryHeap {
                heap: Vec::new(),
                heap_type: HeapType::Min,
            }
        }
        pub fn new(heap_type: HeapType) -> BinaryHeap<T> {
            BinaryHeap {
                heap: Vec::new(),
                heap_type,
            }
        }

        fn comp(&self, c: &T, o: &T) -> bool {
            match self.heap_type {
                HeapType::Min => c < o,
                HeapType::Max => c > o,
            }
        }
        fn percolate(&mut self, direction: PercolateDirection) {
            match direction {
                PercolateDirection::Down => self.percolate_down(),
                PercolateDirection::Up => self.percolate_up(),
            }
        }

        fn percolate_down(&mut self) {
            let mut i = 0;

            while i * 2 < self.heap.len() {
                match self.get_child(i) {
                    Some((child_index, child)) => {
                        if self.comp(child, self.heap.get(i).unwrap()) {
                            self.heap.swap(i, child_index)
                        }
                        i = child_index;
                    }
                    _ => break,
                }
            }
        }

        fn get_child(&self, i: usize) -> Option<(usize, &T)> {
            let (left_index, right_index) =
                (child(i, ChildSide::Left)?, child(i, ChildSide::Right)?);
            let (left_child, right_child) = (self.heap.get(left_index), self.heap.get(right_index));

            if right_child.is_none() && left_child.is_none() {
                None
            } else if right_child.is_none() {
                Some((left_index, left_child.unwrap()))
            } else if left_child.is_none() {
                Some((right_index, right_child.unwrap()))
            } else {
                if self.comp(left_child.unwrap(), right_child.unwrap()) {
                    Some((left_index, left_child.unwrap()))
                } else {
                    Some((right_index, right_child.unwrap()))
                }
            }
        }

        fn percolate_up(&mut self) {
            let mut i = self.heap.len() - 1;

            while i >= 0 {
                match parent(i) {
                    Some(parent_index) => {
                        let c = self.heap.get(i).unwrap();
                        let p = self.heap.get(parent_index).unwrap();
                        if self.comp(c, p) {
                            self.heap.swap(i, parent_index)
                        }

                        i = parent_index
                    }
                    None => break,
                }
            }
        }

        pub fn insert(&mut self, item: T) {
            self.heap.push(item);
            self.percolate(PercolateDirection::Up)
        }

        pub fn pop(&mut self) -> T {
            let popped = self.heap.remove(0);

            if self.heap.len() > 0 {
                let last = self.heap.remove(self.heap.len() - 1);
                self.heap.insert(0, last);

                self.percolate(PercolateDirection::Down);
            }

            popped
        }
    }
}

#[cfg(test)]
mod tests {

    use super::heap::*;

    #[test]
    fn test_new() {
        let heap: BinaryHeap<String> = BinaryHeap::new(HeapType::Max);
    }

    #[test]
    fn test_default() {
        let heap: BinaryHeap<String> = BinaryHeap::default();
    }

    #[test]
    fn test_heaps() {
        use itertools::Itertools;
        use rand::Rng;

        let mut v = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..100000 {
            v.push(rng.gen::<u8>() as u8);
        }
        v = v.into_iter().unique().collect();
        let mut sorted_v = v.clone();
        sorted_v.sort();

        let mut heap: BinaryHeap<u8> = BinaryHeap::default();

        for i in v.iter() {
            heap.insert(*i);
        }

        let mut res = Vec::new();
        for _ in 0..v.len() {
            res.push(heap.pop());
        }

        assert_eq!(sorted_v, res);
    }

    #[test]
    fn test_parent() {
        assert!(parent(0).is_none());
        assert_eq!(parent(1).unwrap(), 0);
        assert_eq!(parent(2).unwrap(), 0);
        assert_eq!(parent(3).unwrap(), 1);
        assert_eq!(parent(6).unwrap(), 2);
        assert_eq!(parent(8).unwrap(), 3);
    }

    #[test]
    fn test_child() {
        assert_eq!(child(0, ChildSide::Left).unwrap(), 1);
        assert_eq!(child(0, ChildSide::Right).unwrap(), 2);
        assert_eq!(child(3, ChildSide::Left).unwrap(), 7);
        assert_eq!(child(3, ChildSide::Right).unwrap(), 8);
        assert_eq!(child(4, ChildSide::Left).unwrap(), 9);
        assert_eq!(child(4, ChildSide::Right).unwrap(), 10);
    }
}
