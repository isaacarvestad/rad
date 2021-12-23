use num::Bounded;
use std::clone::Clone;
use std::cmp::{max, min};
use std::marker::PhantomData;

/// A segment tree is defined by the elements it operates on, the default value
/// of the elements, and the associative function used to combine them.
pub trait SegmentTreeSpec {
    type T: Clone;
    fn default() -> Self::T;
    fn combine(a: Self::T, b: Self::T) -> Self::T;
}

/// Segment tree specification for computing range minimum queries.
pub struct MinSpec<T>(PhantomData<T>);

impl<T: Bounded + Ord + Clone> SegmentTreeSpec for MinSpec<T> {
    type T = T;
    fn default() -> T {
        T::max_value()
    }
    fn combine(a: T, b: T) -> T {
        min(a, b)
    }
}

/// Segment tree specification for computing range maximum queries.
pub struct MaxSpec<T>(PhantomData<T>);

impl<T: Bounded + Ord + Clone> SegmentTreeSpec for MaxSpec<T> {
    type T = T;
    fn default() -> T {
        T::min_value()
    }
    fn combine(a: T, b: T) -> T {
        max(a, b)
    }
}

/// Compute the closest power of two 'x' of 'n' such that 'n <= x'. If 'n' is a
/// power of two then 'x = n'.
fn next_2pow(n: usize) -> usize {
    let mut x = 1;
    while x < n {
        x *= 2
    }
    x
}

/// A data-structure which supports updating and quering sub-ranges over its
/// elements.
pub struct SegmentTree<Spec: SegmentTreeSpec> {
    /// The values of each respective vertex in the tree.
    values: Vec<Spec::T>,
    /// For each vertex in the tree their bound '[l,r)' represents the range it
    /// covers.
    bounds: Vec<(usize, usize)>,
}

impl<Spec: SegmentTreeSpec> SegmentTree<Spec> {
    /// Create a segment tree over 'n' values.
    pub fn new(n: usize) -> Self {
        let tree_size = 2 * next_2pow(n) - 1;
        let mut t = SegmentTree {
            values: vec![Spec::default(); tree_size],
            bounds: vec![(0, 0); tree_size],
        };
        let mut stack = vec![(0, 0, n)];
        while let Some((idx, l, r)) = stack.pop() {
            t.bounds[idx] = (l, r);
            if r - l > 1 {
                let m = l + (r - l) / 2;
                stack.push((idx * 2 + 1, l, m));
                stack.push((idx * 2 + 2, m, r));
            }
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::MinSpec;
    use super::SegmentTree;

    #[test]
    fn it_compiles() {
        let _t = SegmentTree::<MinSpec<i32>>::new(10);
    }

    #[test]
    fn it_sets_up_bounds() {
        let t = SegmentTree::<MinSpec<i32>>::new(3);
        assert_eq!(
            t.bounds,
            vec![(0, 3), (0, 1), (1, 3), (0, 0), (0, 0), (1, 2), (2, 3)]
        )
    }
}
