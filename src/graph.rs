//! Algorithms on graphs.

use num::Zero;
use petgraph::stable_graph::IndexType;
use petgraph::visit::{EdgeRef, IntoEdges, NodeCount, NodeIndexable};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

/// Compute the shortest distance in a graph with non-negative graphs from some
/// source `s` to every vertex in the graph.
///
/// Returns a vector with the distance to each vertex or `None` if vertex cannot
/// be reached.
///
/// Time complexity: `O(|V| log |E|)`.
pub fn shortest_distance<G>(g: G, s: G::NodeId) -> Vec<Option<G::EdgeWeight>>
where
    G: IntoEdges + NodeCount + NodeIndexable,
    G::NodeId: Ord + IndexType,
    G::EdgeWeight: Add<G::EdgeWeight> + Ord + Zero + Copy,
{
    let n = g.node_count();
    let mut distance: Vec<Option<G::EdgeWeight>> = vec![None; n];
    let mut queue = BinaryHeap::new();

    distance[s.index()] = Some(num::zero());
    queue.push(Reverse((num::zero(), s)));

    while let Some(Reverse((d, u))) = queue.pop() {
        if let Some(cur_d) = distance[u.index()] {
            if cur_d < d {
                continue;
            }
        }

        for e in g.edges(u) {
            let new_d = d + *e.weight();
            let should_queue = match distance[e.target().index()] {
                None => true,
                Some(cur_d) => new_d < cur_d,
            };
            if should_queue {
                distance[e.target().index()] = Some(new_d);
                queue.push(Reverse((new_d, e.target())));
            }
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    mod shortest_distance {
        use super::super::shortest_distance;
        use petgraph::graph::Graph;
        use petgraph::graph::UnGraph;

        #[test]
        fn singleton_graph() {
            let mut g: UnGraph<(), usize> = Graph::new_undirected();
            g.add_node(());
            assert_eq!(shortest_distance(&g, 0.into()), vec![Some(0)]);
        }

        #[test]
        fn small_path_graph() {
            let g: UnGraph<(), usize> = Graph::from_edges(&[(0, 1, 5), (1, 2, 10)]);
            assert_eq!(
                shortest_distance(&g, 0.into()),
                vec![Some(0), Some(5), Some(15)]
            )
        }

        #[test]
        fn two_component_graph() {
            let g: UnGraph<(), usize> =
                Graph::from_edges(&[(0, 1, 5), (1, 2, 3), (0, 2, 7), (3, 4, 0)]);
            assert_eq!(
                shortest_distance(&g, 0.into()),
                vec![Some(0), Some(5), Some(7), None, None]
            );
        }

        #[test]
        fn cycle_graph() {
            let g: UnGraph<(), usize> =
                Graph::from_edges(&[(0, 1, 2), (1, 2, 2), (2, 3, 2), (3, 0, 2)]);
            assert_eq!(
                shortest_distance(&g, 0.into()),
                vec![Some(0), Some(2), Some(4), Some(2)]
            )
        }

        #[test]
        fn simple_choice() {
            let g: UnGraph<(), usize> = Graph::from_edges(&[(0, 1, 10), (0, 2, 50), (1, 2, 10)]);
            assert_eq!(
                shortest_distance(&g, 0.into()),
                vec![Some(0), Some(10), Some(20)]
            )
        }
    }
}
