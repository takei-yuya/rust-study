use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

/// 二分ヒープ
///
/// 値を登録し、小さい順に値を取り出すデータ構造。 a.k.a. 優先度付きキュー
///
/// # Examples
///
/// ```
/// use rust_study::collections::heap::Heap;
/// let mut heap: Heap<i32> = Heap::new();
/// assert!(heap.is_empty());
/// assert_eq!(0, heap.len());
/// assert_eq!(None, heap.peek());
///
/// vec![1, 7, 3, 5].iter_mut().for_each(|i| heap.push(*i) );
/// assert!(!heap.is_empty());
/// assert_eq!(4, heap.len());
///
/// assert_eq!(Some(&1), heap.peek());
/// assert_eq!(Some(1), heap.pop());
/// assert_eq!(Some(3), heap.pop());
///
/// vec![2, 8, 4, 6].iter_mut().for_each(|i| heap.push(*i) );
/// assert_eq!(6, heap.len());
///
/// assert_eq!(vec![2, 4, 5], heap.drain(3));
/// assert_eq!(vec![6, 7, 8], heap.drain(5));
/// assert!(heap.is_empty());
/// assert_eq!(0, heap.len())
/// ```

pub struct Heap<T> {
    heap: Vec<T>,
    compare: fn(lhs: &T, rhs: &T) -> Ordering,
}

impl <T: Ord> Heap<T> {
    /// 空の二分ヒープを構築します。
    ///
    /// 比較には [`std::cmp::Ord::cmp()`] が使われます。
    pub fn new() -> Self {
        Heap {
            heap: vec![],
            compare: Ord::cmp,
        }
    }
}

impl <T> Heap<T> {
    /// 空の二分ヒープを構築します。
    ///
    /// 比較には与えられた関数が使われます。
    pub fn with_compare(compare: fn(lhs: &T, rhs: &T) -> Ordering) -> Self {
        Heap {
            heap: vec![],
            compare,
        }
    }

    /// 要素を二分ヒープに追加します。
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn push(&mut self, v: T) {
        self.heap.push(v);
        self.heap_up(self.len() - 1);
    }

    /// 二分ヒープから最も小さい値を取り除きます。空の場合、 `None` を返します。
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.heap.swap_remove(0);
        self.heap_down(0);
        Some(result)
    }

    /// 二分ヒープの一番小さい値を参照します。空の場合、 `None` を返します。
    pub fn peek(&mut self) -> Option<&T> {
        self.heap.first()
    }

    /// 二分ヒープが空の場合に、 `true` を返します。
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// 二分ヒープの要素数を返します。
    pub fn len(&self) -> usize { self.heap.len() }

    /// 要素を保持するための内部の配列の容量を確保します。
    ///
    /// [`Vec::reserve()`] を参照してください。
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn reserve(&mut self, additional: usize) { self.heap.reserve(additional) }

    /// 要素を保持するための内部の配列の容量を確保します。
    ///
    /// [`Vec::reserve_exact()`] を参照してください。
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn reserve_exact(&mut self, additional: usize) { self.heap.reserve_exact(additional) }

    /// `num` で指定した件数を上限に、小さい順にヒープから取り除き `Vec<T>` として返します。
    pub fn drain(&mut self, num: usize) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        for _ in 0..num {
            if let Some(v) = self.pop() {
                vec.push(v);
            } else {
                break;
            }
        }
        vec
    }

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
        heap.push(2);                       assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(4);                       assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(3);                       assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(2), heap.pop());    assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(3), heap.pop());    assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(1);                       assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(5);                       assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(1), heap.pop());    assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(4), heap.pop());    assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(5), heap.pop());    assert_eq!(0, heap.len()); assert!(heap.is_empty());
    }

    #[test]
    fn with_compare() {
        // Reverse order
        let mut heap = Heap::<i32>::with_compare(|lhs,rhs| rhs.cmp(lhs));
                                            assert_eq!(0, heap.len()); assert!(heap.is_empty());
        heap.push(2);                       assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(4);                       assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(3);                       assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(4), heap.pop());    assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(3), heap.pop());    assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        heap.push(1);                       assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        heap.push(5);                       assert_eq!(3, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(5), heap.pop());    assert_eq!(2, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(2), heap.pop());    assert_eq!(1, heap.len()); assert!(!heap.is_empty());
        assert_eq!(Some(1), heap.pop());    assert_eq!(0, heap.len()); assert!(heap.is_empty());
    }
}
