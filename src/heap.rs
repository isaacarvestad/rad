pub struct Heap<T: Ord> {
    xs: Vec<T>,
}

impl<T: Ord> Heap<T> {
    /// The parent of index 'i' in a perfectly balanced binary tree or 'None' if
    /// 'i' is the root.
    fn parent(i: usize) -> Option<usize> {
        if i == 0 {
            return None;
        } else {
            return Some((i - 1) / 2);
        }
    }

    /// The left child index 'i' in a perfectly balanced binary tree or 'None'
    /// if this access is out of bounds.
    fn left_child(&self, i: usize) -> Option<usize> {
        let j = 2 * i + 1;
        if j >= self.xs.len() {
            return None;
        } else {
            return Some(j);
        }
    }

    /// The right child index 'i' in a perfectly balanced binary tree or 'None'
    /// if this access is out of bounds.
    fn right_child(&self, i: usize) -> Option<usize> {
        let j = 2 * i + 2;
        if j >= self.xs.len() {
            return None;
        } else {
            return Some(j);
        }
    }

    /// Create an empty max-heap.
    ///
    /// Time complexity: O(1)
    pub fn new() -> Self {
        Heap { xs: Vec::new() }
    }

    /// True if heap is empty, false otherwise.
    ///
    /// Time complexity: O(1)
    pub fn empty(&self) -> bool {
        self.xs.len() == 0
    }

    /// Insert an element into the max-heap.
    ///
    /// Time complexity: O(log n)
    pub fn push(&mut self, x: T) {
        self.xs.push(x);
        let mut i = self.xs.len() - 1;
        while let Some(j) = Heap::<T>::parent(i) {
            if self.xs[i] > self.xs[j] {
                self.xs.swap(i, j);
                i = j;
            } else {
                break;
            }
        }
    }

    /// Get a reference to the largest element in the heap or 'None' if heap is
    /// empty.
    ///
    /// Time complexity: O(1)
    pub fn peek(&self) -> Option<&T> {
        if self.xs.len() == 0 {
            return None;
        } else {
            return Some(&self.xs[0]);
        }
    }

    /// Remove the largest element in the heap. Returns the removed element or
    /// 'None' in case the heap was empty.
    ///
    /// Time complexity: O(log n)
    pub fn pop(&mut self) -> Option<T> {
        if self.xs.len() == 0 {
            return None;
        } else {
            let n = self.xs.len();
            self.xs.swap(0, n - 1);
            let popped_elem = self.xs.pop();

            let mut i = 0;
            while let Some(l) = self.left_child(i) {
                if let Some(r) = self.right_child(i) {
                    if self.xs[l] < self.xs[r] {
                        if self.xs[i] < self.xs[r] {
                            self.xs.swap(i, r);
                            i = r;
                        } else {
                            break;
                        }
                    } else {
                        if self.xs[i] < self.xs[l] {
                            self.xs.swap(i, l);
                            i = l;
                        } else {
                            break;
                        }
                    }
                } else {
                    if self.xs[i] < self.xs[l] {
                        self.xs.swap(i, l);
                        i = l;
                    } else {
                        break;
                    }
                }
            }
            popped_elem
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn it_constructs_empty() {
        let mut heap = Heap::<i32>::new();
        assert!(heap.empty());
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn it_adds_and_removes_single_element() {
        let mut heap = Heap::<i32>::new();
        heap.push(0);
        assert_eq!(heap.xs, vec![0]);
        assert_eq!(heap.peek(), Some(0).as_ref());
        assert_eq!(heap.pop(), Some(0));
        assert!(heap.empty())
    }

    #[test]
    fn it_adds_and_removes_several_elements() {
        let mut heap = Heap::<i32>::new();

        heap.push(1); // 1
        assert_eq!(heap.peek(), Some(1).as_ref());

        heap.push(2); // 2 1
        assert_eq!(heap.peek(), Some(2).as_ref());

        heap.push(0); // 2 1 0
        assert_eq!(heap.peek(), Some(2).as_ref());

        heap.push(5); // 5 2 1 0
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(2));

        heap.push(4); // 4 1 0
        heap.push(0); // 4 1 0 0
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(0));
    }
}
