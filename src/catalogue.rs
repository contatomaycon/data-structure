use std::collections::HashMap;
use crate::models::{Product, ProductId};
use crate::indexer::SearchIndex;
use crate::search::SearchEngine;
use crate::graph::ProductGraph;

pub struct Catalogue {
    products: HashMap<ProductId, Product>,
    index: SearchIndex,
    graph: ProductGraph
}

impl Catalogue {
    pub fn new() -> Self { Self { products: HashMap::new(), index: SearchIndex::new(), graph: ProductGraph::new() } }

    pub fn insert_product(&mut self, p: Product) { self.products.insert(p.id, p); }

    pub fn build_index(&mut self) {
        self.index = SearchIndex::new();
        for p in self.products.values() { self.index.add_product(p); }
        self.index.finalize();
    }

    pub fn build_graph(&mut self) {
        let mut v: Vec<Product> = self.products.values().cloned().collect();
        v.sort_by_key(|p| p.id);
        self.graph.build(&v);
    }

    pub fn search(&self, query: &str) -> Vec<Product> {
        let engine = SearchEngine { index: &self.index };
        let ids = engine.search(query);
        let mut v: Vec<Product> = Vec::new();
        for id in ids {
            if let Some(p) = self.products.get(&id) { v.push(p.clone()); }
        }
        v
    }

    pub fn recommend(&self, id: ProductId, k: usize) -> Vec<Product> {
        let ids = self.graph.recommend(id, k);
        let mut v: Vec<Product> = Vec::new();
        for pid in ids {
            if let Some(p) = self.products.get(&pid) { v.push(p.clone()); }
        }
        v
    }
}