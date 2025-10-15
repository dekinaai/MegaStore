use std::collections::{HashMap, HashSet, VecDeque};
use crate::data::Product;

pub struct SearchIndex {
    pub index: HashMap<String, HashSet<usize>>,
    pub products: HashMap<usize, Product>,
    pub graph: HashMap<usize, HashSet<usize>>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            products: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, p: Product) {
        let id = p.id;
        for token in Self::tokenize(&p.name) {
            self.index.entry(token).or_default().insert(id);
        }
        for token in Self::tokenize(&p.brand) {
            self.index.entry(token).or_default().insert(id);
        }
        for token in Self::tokenize(&p.category) {
            self.index.entry(token).or_default().insert(id);
        }
        self.products.insert(id, p);
        self.graph.entry(id).or_insert_with(HashSet::new);
    }

    pub fn link_products(&mut self, a: usize, b: usize) {
        self.graph.entry(a).or_default().insert(b);
        self.graph.entry(b).or_default().insert(a);
    }

    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let mut results: HashSet<usize> = HashSet::new();
        for token in Self::tokenize(query) {
            if let Some(set) = self.index.get(&token) {
                for &id in set {
                    results.insert(id);
                }
            }
        }
        let mut out: Vec<&Product> = results.into_iter().filter_map(|id| self.products.get(&id)).collect();
        out.sort_by_key(|p| p.id);
        out
    }

    pub fn recommend(&self, product_id: usize, max: usize) -> Vec<&Product> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut recs: Vec<usize> = Vec::new();

        visited.insert(product_id);
        q.push_back(product_id);

        while let Some(node) = q.pop_front() {
            if let Some(neighs) = self.graph.get(&node) {
                for &n in neighs {
                    if !visited.contains(&n) {
                        visited.insert(n);
                        recs.push(n);
                        q.push_back(n);
                        if recs.len() >= max { break; }
                    }
                }
            }
            if recs.len() >= max { break; }
        }

        recs.into_iter().filter_map(|id| self.products.get(&id)).collect()
    }
}
