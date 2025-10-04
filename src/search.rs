use std::collections::HashMap;
use crate::indexer::SearchIndex;
use crate::models::ProductId;
use crate::normalizer::tokenize;

pub struct SearchEngine<'a> {
    pub index: &'a SearchIndex
}

impl<'a> SearchEngine<'a> {
    pub fn search(&self, query: &str) -> Vec<ProductId> {
        let mut scores: HashMap<ProductId, usize> = HashMap::new();
        for token in tokenize(query) {
            if let Some(ids) = self.index.by_name.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 3; }
            }
            if let Some(ids) = self.index.by_category.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 2; }
            }
            if let Some(ids) = self.index.by_tag.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 1; }
            }
        }
        let mut v: Vec<(ProductId, usize)> = scores.into_iter().collect();
        v.sort_by(|a,b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        v.into_iter().map(|(id,_)| id).collect()
    }
}