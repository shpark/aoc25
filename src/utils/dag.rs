use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Either;

pub struct Dag<V>
    where V: PartialEq + Eq + Hash + Clone,
{
    edges: HashMap<V, Vec<V>>
}

impl<V> Dag<V>
    where V: PartialEq + Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            edges: HashMap::new()
        }
    }

    pub fn add_edge<'a>(&mut self, src: &'a V, dst: &'a V) {
        let _ = self.edges.entry(src.clone())
            .and_modify(|dst_nodes| { dst_nodes.push(dst.clone()) })
            .or_insert(vec![dst.clone()]);
    }

    pub fn neighbors<'a>(&self, src: &'a V) -> impl Iterator<Item = V> {
        match self.edges.get(src) {
            Some(neighbors) => Either::Left(neighbors.iter().cloned()),
            None => Either::Right(std::iter::empty()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::dag::Dag;

    #[test]
    fn test_dag_simple() {
        let mut dag = Dag::<i32>::new();

        dag.add_edge(&0, &1);
        dag.add_edge(&0, &2);
        dag.add_edge(&1, &2);

        assert_eq!(dag.neighbors(&0).collect::<Vec<_>>().len(), 2);
        assert_eq!(dag.neighbors(&1).collect::<Vec<_>>().len(), 1);
    }
}