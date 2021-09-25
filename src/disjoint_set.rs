/// A data structure for performing UnionFind queries and updates.
pub struct DisjointSet {
    /// Parent of a given element. If 'parent[u] == u' then element 'u' has no
    /// parent.
    parent: Vec<usize>,
    /// Size of a set. Only valid for the root of a set.
    size: Vec<usize>,
    /// Upper bound on the height of a set. Only valid for the root of a set.
    rank: Vec<usize>,
}

impl DisjointSet {
    /// Construct a new disjoint set with 'n' singleton sets.
    pub fn new(n: usize) -> Self {
        let mut set = DisjointSet {
            parent: vec![0; n],
            size: vec![1; n],
            rank: vec![0; n],
        };
        for u in 0..n {
            set.parent[u] = u
        }
        set
    }

    /// Find the root of element 'u'. If 'u' is the root return itself. Perform
    /// path optimization on the way.
    fn root(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            u
        } else {
            let r = self.root(self.parent[u]);
            self.parent[u] = r;
            r
        }
    }

    /// Join the sets containing 'u' and 'v'. If they are already in the same
    /// set do nothing and return false, otherwise join the sets and return
    /// true.
    pub fn join(&mut self, u: usize, v: usize) -> bool {
        let r1 = self.root(u);
        let r2 = self.root(v);
        if r1 != r2 {
            if self.rank[r1] <= self.rank[r2] {
                if self.rank[r1] == self.rank[r2] {
                    self.rank[r2] += 1
                }
                self.parent[r1] = r2;
                self.size[r2] += self.size[r1];
            } else {
                self.parent[r2] = r1;
                self.size[r1] += self.size[r2]
            }
            true
        } else {
            false
        }
    }

    /// Returns true if 'u' and 'v' are in the same set, otherwise false.
    pub fn same_set(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    /// Returns the size of the set which contains element 'u'.
    pub fn size(&mut self, u: usize) -> usize {
        let r = self.root(u);
        self.size[r]
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;
    #[test]
    fn it_constructs_empty() {
        let set = DisjointSet::new(0);
        assert!(set.parent.is_empty())
    }
    #[test]
    fn it_constructs_small() {
        let n = 10;
        let set = DisjointSet::new(n);
        for u in 0..n {
            assert_eq!(set.parent[u], u)
        }
    }
    #[test]
    fn it_finds_root() {
        let n = 10;
        let mut set = DisjointSet::new(n);
        for u in 0..n {
            assert_eq!(set.root(u), u)
        }
    }
    #[test]
    fn it_joins_sets() {
        let mut set = DisjointSet::new(10);
        assert!(!set.same_set(0, 1));
        assert!(set.join(0, 1));
        assert!(set.same_set(0, 1));

        assert!(!set.same_set(1, 2));
        assert!(set.join(1, 2));
        assert!(set.same_set(1, 2));

        assert!(set.same_set(0, 2));
        assert!(!set.join(0, 2));
    }
    #[test]
    fn it_maintains_set_size() {
        let n = 10;
        let mut set = DisjointSet::new(n);
        for u in 0..n {
            assert_eq!(set.size(u), 1)
        }
        set.join(0, 1);
        assert_eq!(set.size(0), 2);
        assert_eq!(set.size(1), 2);
        for u in 2..n {
            assert_eq!(set.size(u), 1)
        }
        for u in 2..n {
            set.join(u - 1, u);
        }
        for u in 0..n {
            assert_eq!(set.size(u), n)
        }
    }
}
