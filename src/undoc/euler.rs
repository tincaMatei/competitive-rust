struct EulerianCycleSolver<'a, V, E>
where V: Default,
      E: Edge {
    
    graph: &'a Graph<V, E>,
    used_edge: Vec<bool>,
    last_edge: Vec<usize>,
    result: Vec<usize>,
}

impl<'a, V, E> EulerianCycleSolver<'a, V, E>
where V: Default,
      E: Edge {
    
    fn eulerian_dfs(&mut self, node: usize) {
        while self.last_edge[node] < self.graph.adj_list[node].len() {
            let cnt_edge = self.last_edge[node];
            let id = self.graph.adj_list[node][cnt_edge];
            
            if !self.used_edge[id] {
                let other = self.graph.edges[id].to(node);
                self.used_edge[id] = true;

                self.eulerian_dfs(other);
            }

            self.last_edge[node] += 1; 
        }

        self.result.push(node);
    }
}

pub fn solve_eulerian<V, E>(
    graph: &Graph<V, E>,
    undirected: bool,
    cycle: bool
) -> Option<Vec<usize>>
where V: Default,
      E: Edge{
    
    let mut degree = vec![0; graph.v()];

    for node in 0..graph.v() {
        for it in &graph.adj_list[node] {
            let edge = &graph.edges[*it];
            degree[edge.to(node)] += 1;
        }
    }

    let mut start_node = 0;
    let mut cnt_even = 0;
    let mut cnt_pos = 0;
    let mut cnt_neg = 0;

    for node in 0..graph.v() {
        if undirected && degree[node] % 2 == 1 {
            start_node = node;
            cnt_even += 1;
        } else if !undirected && degree[node] < graph.adj_list[node].len() {
            start_node = node;
            cnt_pos += 1;
        } else if !undirected && degree[node] > graph.adj_list[node].len() {
            cnt_neg += 1;
        }
    }

    if undirected {
        if cnt_even > 2 {
            return None;
        }
        if cycle && cnt_even > 0 {
            return None;
        }
    } else {
        if cnt_pos > 1 || cnt_neg > 1 {
            return None;
        }
        if cycle && (cnt_pos > 0 || cnt_neg > 0) {
            return None;
        }
    }
    
    let mut solver = EulerianCycleSolver {
        graph,
        used_edge: vec![false; graph.e()],
        last_edge: vec![0; graph.v()],
        result: Vec::with_capacity(graph.e() + 1),
    };

    solver.eulerian_dfs(start_node);

    if solver.result.len() != graph.e() + 1 {
        None
    } else {
        if !undirected {
            solver.result.reverse();
        }
        Some(solver.result)
    }
}

pub fn find_eulerian_cycle<V, E>(graph: &Graph<V, E>) -> Option<Vec<usize>>
where V: Default,
      E: Edge {
    solve_eulerian(graph, graph.undirected, true)
}

pub fn find_eulerian_path<V, E>(graph: &Graph<V, E>) -> Option<Vec<usize>>
where V: Default,
      E: Edge {
    solve_eulerian(graph, graph.undirected, false)
}

