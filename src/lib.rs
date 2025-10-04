pub mod models;
pub mod normalizer;
pub mod indexer;
pub mod search;
pub mod graph;
pub mod catalogue;

pub mod prelude {
    pub use crate::models::*;
    pub use crate::catalogue::Catalogue;
}