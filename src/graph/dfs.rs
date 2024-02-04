use std::collections::HashMap;

pub struct Dfs<T: Copy + PartialEq> {
    visited: HashMap<T, bool>,
    graph: HashMap<T, Vec<T>>
}

impl<T: Copy + PartialEq> Dfs<T> {
    pub fn new() -> Self {
        return Dfs {
            visited: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    pub fn add(&mut self, u: T, v: T) {
        self.graph.entry(u).or_insert_with(Vec::new).push(v);
    }

    pub fn search(&mut self, u: T) {
        self.visited.insert(u, true);

        match self.graph.get(u) {
            None => {
                return
            }
            Some(ve) => {
                for v in ve {
                    match self.visited.get(v) {
                        None => {
                            self.visited.insert(v, true);
                            self.search(v)
                        }
                        Some(visited) => {
                            if visited {
                                continue
                            }

                            self.visited.insert(v, true);
                            self.search(v)
                        }
                    }
                }
            }
        }
    }
}
