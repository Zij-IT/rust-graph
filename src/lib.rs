#![allow(dead_code)]
pub mod graph {

    pub(crate) struct Graph<V, N> {
        versicles: Vec<Vec<Option<V>>>,
        nodes: Vec<N>,
        versicles_amount: usize,
    }

    impl<V: Clone, N> Graph<V, N> {
        pub fn add_node(&mut self, data: N) -> usize {
            self.nodes.push(data);
            self.versicles.push(vec![None; self.nodes.len() - 1]);
            for v in &mut self.versicles {
                v.push(None);
            }
            self.nodes.len() - 1
        }
        pub fn add_ver(&mut self, n1: usize, n2: usize, data: V) -> Result<(), &str> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                self.versicles_amount += 1;
                self.versicles[n1][n2] = Some(data);
                Ok(())
            } else {
                Err("node number out of range")
            }
        }

        pub fn delate_node(&mut self, node: usize) -> Result<(), &str> {
            if node > self.nodes.len() {
                Err("node number out of range")
            } else {
                self.nodes.remove(node);
                self.versicles.remove(node);
                for v in &mut self.versicles {
                    v.remove(node);
                }
                Ok(())
            }
        }

        pub fn delate_versicles(&mut self, n1: usize, n2: usize) -> Result<(), &str> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                match self.versicles[n1][n2] {
                    None => Err("trying to delate non-existing verticle"),
                    _ => {
                        self.versicles[n1][n2] = None;
                        Ok(())
                    }
                }
            } else {
                Err("node number out of range")
            }
        } 

        pub fn get_nodes_amount(&self) -> usize {
            self.nodes.len()
        }

        pub fn get_versicles_amount(&self) -> usize {
            self.versicles_amount
        }
        //TODO dodać tutaj mutowalne wartości 
        pub fn get_ver_value(&mut self, n1: usize, n2: usize) -> &Option<V> {
            if n1 < self.nodes.len() && n2 < self.nodes.len() {
                &self.versicles[n1][n2]
            } else {
                &None
            }
        }

        pub fn get_node_value(&self, node: usize) -> Option<&N> {
            if node < self.nodes.len() {
                return Some(&self.nodes[node]);
            } else {
                None
            }
        }

        pub fn new() -> Self {
            Graph {
                versicles: Vec::new(),
                nodes: Vec::new(),
                versicles_amount: 0,
            }
        }

    }
}



#[cfg(test)]
mod graph_tests ;