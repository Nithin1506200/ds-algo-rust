#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    weight: u32,
    from: Vertex,
    to: Vertex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }
    fn add_vertex(&mut self, name: &'static str) {
        self.vertices.push(Vertex { name: name });
    }
    fn get_vertices(&self, name: &str) -> &Vertex {
        self.vertices.iter().find(|x| x.name == name).unwrap()
    }
    // fn add_edge(&mut self, from: &str, to: &str, weight: u32) {
    //     let from = self.get_vertices(from);
    //     let to = self.get_vertices(to);
    //     self.edges
    // }
}
