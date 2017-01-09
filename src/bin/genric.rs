

trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> i32{
    32i32
}


struct Node;

struct Edge;

struct SimpleGraph;

impl Graph for SimpleGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        vec![]
    }

}

fn main() {
    let graph = SimpleGraph;
    let object = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;
}
