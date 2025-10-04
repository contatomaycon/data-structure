use petgraph::visit::EdgeRef;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use crate::models::{Product, ProductId};
use crate::normalizer::normalize_token;

pub struct ProductGraph {
    g: DiGraph<ProductId, f32>,
    idx_by_id: HashMap<ProductId, NodeIndex>,
}

impl ProductGraph {
    pub fn new() -> Self { Self { g: DiGraph::new(), idx_by_id: HashMap::new() } }

    pub fn build(&mut self, products: &[Product]) {
        self.g = DiGraph::new();
        self.idx_by_id.clear();
        for p in products {
            let idx = self.g.add_node(p.id);
            self.idx_by_id.insert(p.id, idx);
        }
        for i in 0..products.len() {
            for j in i+1..products.len() {
                let a = &products[i];
                let b = &products[j];
                let w = self.weight(a,b);
                if w > 0.0 {
                    let ia = self.idx_by_id[&a.id];
                    let ib = self.idx_by_id[&b.id];
                    self.g.add_edge(ia, ib, w);
                    self.g.add_edge(ib, ia, w);
                }
            }
        }
    }

    fn weight(&self, a: &Product, b: &Product) -> f32 {
        let ca = normalize_token(&a.category);
        let cb = normalize_token(&b.category);
        let mut w = 0.0;
        if ca == cb { w += 2.0; }
        let sa: std::collections::HashSet<_> = a.tags.iter().map(|t| normalize_token(t)).collect();
        let sb: std::collections::HashSet<_> = b.tags.iter().map(|t| normalize_token(t)).collect();
        let inter = sa.intersection(&sb).count();
        if inter > 0 { w += inter as f32; }
        w
    }

    pub fn recommend(&self, id: ProductId, k: usize) -> Vec<ProductId> {
        let mut out: Vec<(ProductId,f32)> = Vec::new();
        if let Some(&idx) = self.idx_by_id.get(&id) {
            for e in self.g.edges(idx) {
                out.push((*self.g.node_weight(e.target()).unwrap(), *e.weight()));
            }
            out.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
            out.truncate(k);
            return out.into_iter().map(|(id,_)| id).collect();
        }
        Vec::new()
    }
}