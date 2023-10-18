pub mod heap {

    /// A BinaryHeap is represnted here.
    /// Binary heap adhere to the following properties:
    /// (i) Each node can have up to 2 children (ChildSide::left & ChildSide::Right).
    /// (ii) For any given node, its children must either be smaller if heap_type=HeapType::Min or
    /// larger if heap_type=HeapType::Bigger than the parent.
    /// (iii) Any node populates first its ChildSide::left and then its ChildSide::Right.

    pub struct BinaryHeap<T> {
        /// A binary heap must have a HeapType.
        pub heap_type: HeapType,
        heap: Vec<T>,
    }

    /// A HeapType is represented here.
    /// This can be either Min/Max.
    #[derive(Clone)]
    pub enum HeapType {
        Min,
        Max,
    }

    // ChildSide represents the children side of the binary heap.
    // Each noe in a binary heap contains up to 2 children.
    enum ChildSide {
        Left,
        Right,
    }

    // Percolation direction is represented here.
    // Percolations ensure these properties are maintained as we add/remove nodes from the
    // BinaryHeap.
    enum PercolateDirection {
        Up,
        Down,
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
        pub fn new(heap_type: HeapType) -> Self {
            BinaryHeap {
                heap: Vec::new(),
                heap_type,
            }
        }
        pub fn size(&self) -> usize {
            self.heap.len()
        }
        pub fn peek(&self) -> Option<&T> {
            self.heap.get(0)
        }

        pub fn clone(&self) -> Self {
            BinaryHeap {
                heap: self.heap.clone(),
                heap_type: self.heap_type.clone(),
            }
        }
        fn parent_index(&self, i: usize) -> Option<usize> {
            if i <= 0 {
                // no parent
                None
            } else if i % 2 == 1 {
                // left
                Some(i / 2)
            } else {
                // right
                Some(i / 2 - 1)
            }
        }

        fn child_index(&self, i: usize, side: ChildSide) -> usize {
            match side {
                ChildSide::Left => (i * 2) + 1,
                ChildSide::Right => (i * 2) + 2,
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
            // Logic:
            // (i) Set current as root of BinaryHeap.
            // (ii) Get the smallest/biggest child.
            // (iii) Swap if smaller/bigger than current.
            // (iv) Set current as child
            // (v) Repeat (ii)-(iv) until no more children
            let mut i = 0;

            // still have children
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
            let (left_index, right_index) = (
                self.child_index(i, ChildSide::Left),
                self.child_index(i, ChildSide::Right),
            );

            match (self.heap.get(left_index), self.heap.get(right_index)) {
                (None, None) => None,
                (Some(left), None) => Some((left_index, left)),
                (None, Some(right)) => Some((right_index, right)),
                (Some(left), Some(right)) => {
                    if self.comp(left, right) {
                        Some((left_index, left))
                    } else {
                        Some((right_index, right))
                    }
                }
            }
        }

        fn percolate_up(&mut self) {
            // Logic:
            // (i) Set current as last item of BinaryHeap.
            // (ii) Get parent.
            // (iii) Swap if bigger/smaller than current.
            // (iv) Set current as parent
            // (v) Repeat (ii)-(iv) until reach parent
            let mut i = self.heap.len() - 1;

            loop {
                match self.parent_index(i) {
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

        pub fn pop(&mut self) -> Option<T> {
            match self.heap.len() {
                0 => None,
                _ => {
                    let popped = self.heap.remove(0);
                    if self.heap.len() > 0 {
                        let last = self.heap.remove(self.heap.len() - 1);
                        self.heap.insert(0, last);

                        self.percolate(PercolateDirection::Down);
                    }

                    Some(popped)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_new() {
            BinaryHeap::<String>::new(HeapType::Max);
        }

        #[test]
        fn test_default() {
            BinaryHeap::<String>::default();
        }

        #[test]
        fn test_pop() {
            let mut b = BinaryHeap::<i32>::default();

            assert!(b.pop().is_none());

            b.insert(2);
            assert_eq!(b.pop(), Some(2));
        }

        #[test]
        fn test_size() {
            let mut b = BinaryHeap::<i32>::default();

            assert!(b.pop().is_none());

            b.insert(2);
            assert_eq!(b.size(), 1);
        }

        #[test]
        fn test_heaps() {
            use rand::Rng;

            let mut v = Vec::new();
            let mut rng = rand::thread_rng();

            for _ in 0..100000 {
                v.push(rng.gen::<u8>());
            }
            let mut heap: BinaryHeap<u8> = BinaryHeap::default();

            // test - insert and pop
            for i in v.iter() {
                heap.insert(*i);
            }

            let mut res = Vec::new();
            for _ in 0..v.len() {
                res.push(heap.pop().unwrap());
            }

            // expected
            let mut sorted_v = v.clone();
            sorted_v.sort();

            // asserts
            assert_eq!(sorted_v, res);
        }

        #[test]
        fn test_index() {
            let heap = BinaryHeap::<u8>::default();
            assert!(heap.parent_index(0).is_none());
            assert_eq!(heap.parent_index(1).unwrap(), 0);
            assert_eq!(heap.parent_index(2).unwrap(), 0);
            assert_eq!(heap.parent_index(3).unwrap(), 1);
            assert_eq!(heap.parent_index(6).unwrap(), 2);
            assert_eq!(heap.parent_index(8).unwrap(), 3);
        }

        #[test]
        fn test_child() {
            let heap = BinaryHeap::<u8>::default();
            assert_eq!(heap.child_index(0, ChildSide::Left), 1);
            assert_eq!(heap.child_index(0, ChildSide::Right), 2);
            assert_eq!(heap.child_index(3, ChildSide::Left), 7);
            assert_eq!(heap.child_index(3, ChildSide::Right), 8);
            assert_eq!(heap.child_index(4, ChildSide::Left), 9);
            assert_eq!(heap.child_index(4, ChildSide::Right), 10);
        }
    }
}
