use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

pub struct Heap<T> {
    heap: Vec<T>,
    compare: fn(lhs: &T, rhs: &T) -> Ordering,
}

impl <T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap {
            heap: vec![],
            compare: Ord::cmp,
        }
    }
}

impl <T> Heap<T> {
    pub fn with_compare(compare: fn(lhs: &T, rhs: &T) -> Ordering) -> Self {
        Heap {
            heap: vec![],
            compare,
        }
    }

    pub fn push(&mut self, v: T) {
        self.heap.push(v);
        self.heap_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> T  {
        assert!(!self.is_empty());
        let result = self.heap.swap_remove(0);
        self.heap_down(0);
        result
    }

    pub fn is_empty(&self) -> bool { self.heap.is_empty() }
    pub fn len(&self) -> usize { self.heap.len() }
    pub fn reserve(&mut self, additional: usize) { self.heap.reserve(additional) }

    fn heap_up(&mut self, i: usize) {
        if i == 0 { return; }
        let parent = (i - 1) / 2;
        if (self.compare)(&self.heap[i], &self.heap[parent]) == Less {
            self.heap.swap(i, parent);
            self.heap_up(parent);
        }
    }

    fn heap_down(&mut self, i: usize) {
        let mut child = i * 2 + 1;
        if child >= self.len() { return; }
        let right = child + 1;
        if right < self.len() && (self.compare)(&self.heap[right], &self.heap[child]) == Less {
            child = right;
        }
        if (self.compare)(&self.heap[child], &self.heap[i]) == Less {
            self.heap.swap(i, child);
            self.heap_down(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut heap = Heap::new();
                                        assert_eq!(0, heap.len()); assert!(heap.is_empty());
        heap.push(2);                   assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(4);                   assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(3);                   assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(2, heap.pop());      assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(3, heap.pop());      assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(1);                   assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(5);                   assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(1, heap.pop());      assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(4, heap.pop());      assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        assert_eq!(5, heap.pop());      assert_eq!(0, heap.len()); assert!(heap.is_empty());
    }

    #[test]
    fn with_compare() {
        let mut heap = Heap::<i32>::with_compare(|lhs,rhs| rhs.cmp(lhs));
                                        assert_eq!(0, heap.len()); assert!(heap.is_empty());
        heap.push(2);                   assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(4);                   assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(3);                   assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(4, heap.pop());      assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(3, heap.pop());      assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(1);                   assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(5);                   assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(5, heap.pop());      assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(2, heap.pop());      assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        assert_eq!(1, heap.pop());      assert_eq!(0, heap.len()); assert!(heap.is_empty());
    }
}
