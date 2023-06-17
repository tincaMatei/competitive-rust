pub mod adj_list;

use std::slice::Iter;

/// A trait for representing graphs, regardless of whether they're directed or undirected.
pub trait Graph<'a, T>
where T: 'a + Copy {
    /// Returns the number of nodes in the graph.
    fn v(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn e(&self) -> usize;
    
    /// Returns the neighbours of a node as an iterator.
    fn neighbours(&'a self, node: T) -> Iter<'a, T>;

    /// Returns the set of nodes.
    fn nodes(&self) -> Vec<T>;

    /// Returns the set of edges;
    fn edges(&'a self) -> Vec<(T, T)> {
        let mut edges = Vec::<(T, T)>::new();

        for node in self.nodes() {
            for neigh in self.neighbours(node) {
                edges.push((node, *neigh));
            }
        }

        edges
    }
}

