pub type Vertex = usize;

/// An undirected graph where edges are represented by adjacency lists.
pub struct Graph {
    adjacent: Vec<Vec<Vertex>>,
}

impl Graph {
    /// Construct a graph with 'n' vertices and no edges.
    pub fn new(n: usize) -> Self {
        Graph {
            adjacent: vec![Vec::new(); n],
        }
    }

    /// Number of vertices in graph.
    pub fn size(&self) -> usize {
        self.adjacent.len()
    }

    /// The edges adjacent to a vertex.
    pub fn edges(&self, u: usize) -> &[usize] {
        self.adjacent[u].as_slice()
    }

    /// Add an undirected edge between 'u' and 'v'
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacent[u].push(v);
        self.adjacent[v].push(u);
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn it_constructs_empty() {
        let g = Graph::new(0);
        assert_eq!(g.size(), 0)
    }

    #[test]
    fn it_constructs_singleton() {
        let g = Graph::new(1);
        assert_eq!(g.size(), 1)
    }

    #[test]
    fn it_adds_edges() {
        let mut g = Graph::new(5);
        assert_eq!(g.size(), 5);
        for u in 0..g.size() {
            assert_eq!(g.edges(u).len(), 0)
        }
        g.add_edge(1, 2);
        assert_eq!(g.edges(1), &[2]);
        assert_eq!(g.edges(2), &[1]);
    }
}
