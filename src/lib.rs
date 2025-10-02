use std::collections::{HashMap, HashSet, VecDeque};

pub type ProductId = usize;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ProductGraph {
    adjacency: HashMap<ProductId, HashSet<ProductId>>,
}

impl ProductGraph {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: ProductId, b: ProductId) {
        self.adjacency.entry(a).or_default().insert(b);
        self.adjacency.entry(b).or_default().insert(a);
    }

    pub fn neighbours(&self, id: ProductId) -> HashSet<ProductId> {
        self.adjacency.get(&id).cloned().unwrap_or_default()
    }

    pub fn recommendations(&self, id: ProductId, max_depth: usize) -> Vec<ProductId> {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(ProductId, usize)> = VecDeque::new();
        let mut results = Vec::new();

        visited.insert(id);
        queue.push_back((id, 0));

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }
            for neighbour in self.neighbours(current) {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    results.push(neighbour);
                    queue.push_back((neighbour, depth + 1));
                }
            }
        }
        results
    }
}

#[derive(Default)]
pub struct Catalogue {
    products: HashMap<ProductId, Product>,
    name_index: HashMap<String, ProductId>,
    graph: ProductGraph,
}

impl Catalogue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_product(&mut self, product: Product) {
        let id = product.id;
        self.name_index.insert(product.name.to_lowercase(), id);
        self.products.insert(id, product);
    }

    pub fn build_graph(&mut self) {
        let ids: Vec<ProductId> = self.products.keys().cloned().collect();
        for (i, &id_a) in ids.iter().enumerate() {
            for &id_b in ids.iter().skip(i + 1) {
                if let (Some(a), Some(b)) = (self.products.get(&id_a), self.products.get(&id_b)) {
                    if a.category == b.category
                        || !a.tags.is_empty() && !b.tags.is_empty()
                            && a.tags.iter().any(|t| b.tags.contains(t))
                    {
                        self.graph.add_edge(id_a, id_b);
                    }
                }
            }
        }
    }

    pub fn search(&self, query: &str) -> impl Iterator<Item = &Product> {
        let query_lower = query.to_lowercase();
        self.products
            .values()
            .filter(move |product| product.name.to_lowercase().contains(&query_lower))
    }

    pub fn recommend(&self, id: ProductId, max_depth: usize) -> Vec<&Product> {
        let mut ids = self.graph.recommendations(id, max_depth);
        ids.sort_unstable();
        ids.iter()
            .filter_map(|&pid| self.products.get(&pid))
            .collect()
    }
}

