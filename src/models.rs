use serde::{Serialize, Deserialize};

pub type ProductId = usize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
}