// https://www.w3schools.com/dsa/dsa_algo_graphs_dijkstra.php

use std::{collections::HashSet, usize};

struct Graph {
    size: usize,
    adjacent_matrix: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            size,
            adjacent_matrix: vec![vec![usize::MAX; size]],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, weight: usize) -> Result<(), ()> {
        if u >= self.size || v >= self.size {
            Err(())
        } else {
            self.adjacent_matrix[u][v] = weight;
            Ok(())
        }
    }
    pub fn dijkstra(&self, u: usize) -> Vec<usize> {
        let mut distance = vec![usize::MAX, self.size];
        let mut visited: HashSet<usize> = HashSet::new();
        distance[u] = 0;
        for _ in 0..self.size {
            let mut min_distance = usize::MAX;
            let mut ind: Option<usize> = None;
            for i in 0..self.size {
                if visited.contains(&i) && distance[i] < min_distance {
                    min_distance = distance[i];
                    ind = Some(i)
                }
            }
            match ind {
                Some(indx) => {
                    visited.insert(indx);
                    for v in 0..self.size {
                        if self.adjacent_matrix[indx][v] != 0 && !visited.contains(&v) {
                            let alt = distance[indx] + self.adjacent_matrix[indx][v];
                            if alt < distance[v] {
                                distance[v] = alt
                            }
                        }
                    }
                }
                None => break,
            }
        }
        distance
    }
}
