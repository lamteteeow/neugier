use std::vec;

pub struct Node {
    pub f: f32,
    pub df: f32,
    parents: Vec<(usize, f32)>,
}

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, f: f32) -> usize {
        self.nodes.push(Node {
            f,
            df: 0.,
            parents: vec![],
        });
        self.nodes.len() - 1
    }

    pub fn add(&mut self, a: usize, b: usize) -> usize {
        let v_a = self.nodes[a].f;
        let v_b = self.nodes[b].f;
        self.nodes.push(Node {
            f: v_a + v_b,
            df: 0.,
            parents: vec![(a, 1.), (b, 1.)],
        });
        self.nodes.len() - 1
    }

    pub fn sub(&mut self, a: usize, b: usize) -> usize {
        let v_a = self.nodes[a].f;
        let v_b = self.nodes[b].f;
        self.nodes.push(Node {
            f: v_a - v_b,
            df: 0.,
            parents: vec![(a, 1.), (b, -1.)],
        });
        self.nodes.len() - 1
    }

    pub fn mul(&mut self, a: usize, b: usize) -> usize {
        let v_a = self.nodes[a].f;
        let v_b = self.nodes[b].f;
        self.nodes.push(Node {
            f: v_a * v_b,
            df: 0.,
            parents: vec![(a, v_b), (b, v_a)],
        });
        self.nodes.len() - 1
    }

    pub fn div(&mut self, a: usize, b: usize) -> usize {
        let v_a = self.nodes[a].f;
        let v_b = self.nodes[b].f;
        self.nodes.push(Node {
            f: v_a / v_b,
            df: 0.,
            parents: vec![(a, 1. / v_b), (b, -v_a / (v_b * v_b))],
        });
        self.nodes.len() - 1
    }

    pub fn sin(&mut self, a: usize) -> usize {
        let v_a = self.nodes[a].f;
        self.nodes.push(Node {
            f: v_a.sin(),
            df: 0.,
            parents: vec![(a, v_a.cos())],
        });
        self.nodes.len() - 1
    }

    pub fn cos(&mut self, a: usize) -> usize {
        let v_a = self.nodes[a].f;
        self.nodes.push(Node {
            f: v_a.cos(),
            df: 0.,
            parents: vec![(a, -v_a.sin())],
        });
        self.nodes.len() - 1
    }

    pub fn exp(&mut self, a: usize) -> usize {
        let v_a = self.nodes[a].f;
        self.nodes.push(Node {
            f: v_a.exp(),
            df: 0.,
            parents: vec![(a, v_a.exp())],
        });
        self.nodes.len() - 1
    }

    pub fn backward(&mut self, target: usize) {
        self.nodes[target].df = 1.0;

        for i in (0..=target).rev() {
            let df_i = self.nodes[i].df;
            let parents = self.nodes[i].parents.clone();
            for (parent_id, edge_weight) in parents {
                self.nodes[parent_id].df += df_i * edge_weight;
            }
        }
    }
}
