use petgraph::graph::{NodeIndex, UnGraph};

/// Answers lowest common ancestor queries in O(log n) time.
#[derive(Debug)]
pub struct LCA {
    jump_table: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    /// Compute base 2 logarithm of 'n' rounded up to next integer.
    ///
    /// Time complexity: O(log n)
    fn log2(n: usize) -> usize {
        let mut x = 1;
        let mut count = 0;
        while x < n {
            x *= 2;
            count += 1;
        }
        count
    }

    /// Construct a LCA data structure given an undirected graph representing a
    /// tree (no cycles) and a vertex to root the tree from.
    pub fn new(graph: UnGraph<(), ()>, root: NodeIndex) -> Self {
        let n = graph.node_count();
        let levels = LCA::log2(n);
        let mut lca = LCA {
            jump_table: vec![vec![usize::MAX; n]; levels],
            depth: vec![usize::MAX; n],
        };

        // Init jump table by BFS from root.
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        lca.depth[root.index()] = 0;

        // Explore a vertex 'u' in the BFS.
        let mut explore = |u| {
            let mut to_explore = Vec::new();
            for v in graph.neighbors(u) {
                if lca.depth[v.index()] == usize::MAX {
                    to_explore.push(v);
                    lca.depth[v.index()] = lca.depth[u.index()] + 1;
                    lca.jump_table[0][v.index()] = u.index();
                    for i in 1..levels {
                        let ancestor = lca.jump_table[i - 1][v.index()];
                        if ancestor != usize::MAX {
                            lca.jump_table[i][v.index()] = lca.jump_table[i - 1][ancestor]
                        }
                    }
                }
            }
            to_explore
        };

        // BFS loop
        loop {
            match queue.pop_front() {
                None => break,
                Some(u) => explore(u).iter().for_each(|&v| queue.push_back(v)),
            }
        }
        lca
    }

    /// Jump upwards 'k' steps starting from vertex 'u'.
    fn jump(&self, mut u: usize, k: usize) -> usize {
        let levels = self.jump_table.len();
        for i in (0..levels).rev() {
            if u != usize::MAX && (k & (1 << i)) != 0 {
                u = self.jump_table[i][u]
            }
        }
        u
    }

    /// Compute the lowest common ancestor of vertices 'u' and 'v'.
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        // Jump in tree such that 'u' and 'v' are at equal depth.
        if self.depth[u] < self.depth[v] {
            v = self.jump(v, self.depth[v] - self.depth[u])
        }
        if self.depth[u] > self.depth[v] {
            u = self.jump(u, self.depth[u] - self.depth[v])
        }
        // If equal after initial jump then return either 'u' or 'v'.
        if u == v {
            return u;
        }
        let levels = self.jump_table.len();
        // Binary search for lowest height such that 'u != v'.
        for i in (0..levels).rev() {
            let pu = self.jump_table[i][u];
            let pv = self.jump_table[i][v];
            if pu != usize::MAX && pv != usize::MAX && pu != pv {
                u = pu;
                v = pv;
            }
        }
        self.jump_table[0][u]
    }
}

#[cfg(test)]
mod tests {
    use super::LCA;
    use petgraph::graph::UnGraph;

    /// Construct the following graph:
    ///
    ///          0
    ///        /   \
    ///       1     2
    ///      / \   / \
    ///     3   4 5   6
    fn simple_graph() -> UnGraph<(), ()> {
        UnGraph::from_edges(&[(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)])
    }

    #[test]
    fn it_answers_simple_queries() {
        let g = simple_graph();
        let solver = LCA::new(g, 0.into());

        assert_eq!(solver.lca(0, 1), 0);
        assert_eq!(solver.lca(0, 3), 0);
        assert_eq!(solver.lca(1, 2), 0);
        assert_eq!(solver.lca(4, 1), 1);
        assert_eq!(solver.lca(3, 4), 1);
        assert_eq!(solver.lca(5, 6), 2);
        assert_eq!(solver.lca(3, 5), 0);
    }
}
