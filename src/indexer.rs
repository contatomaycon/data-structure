use std::collections::HashMap;
use crate::models::{Product, ProductId};
use crate::normalizer::{tokenize, normalize_token};

#[derive(Default)]
pub struct SearchIndex {
    pub by_name: HashMap<String, Vec<ProductId>>,
    pub by_category: HashMap<String, Vec<ProductId>>,
    pub by_tag: HashMap<String, Vec<ProductId>>,
}

impl SearchIndex {
    pub fn new() -> Self { Self::default() }

    pub fn add_product(&mut self, p: &Product) {
        for t in tokenize(&p.name) {
            self.by_name.entry(t).or_default().push(p.id);
        }
        let c = normalize_token(&p.category);
        self.by_category.entry(c).or_default().push(p.id);
        for tag in &p.tags {
            let t = normalize_token(tag);
            self.by_tag.entry(t).or_default().push(p.id);
        }
    }

    pub fn finalize(&mut self) {
        for v in self.by_name.values_mut() { v.sort_unstable(); v.dedup(); }
        for v in self.by_category.values_mut() { v.sort_unstable(); v.dedup(); }
        for v in self.by_tag.values_mut() { v.sort_unstable(); v.dedup(); }
    }
}