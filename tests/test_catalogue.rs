use megastore_search::{Catalogue, Product};

#[test]
fn search_finds_matching_products() {
    let mut cat = Catalogue::new();
    cat.insert_product(Product {
        id: 1,
        name: "Rust Book".to_string(),
        category: "Books".to_string(),
        tags: vec!["programming".to_string()],
    });
    cat.insert_product(Product {
        id: 2,
        name: "Cooking for Beginners".to_string(),
        category: "Books".to_string(),
        tags: vec!["cooking".to_string()],
    });
    let results: Vec<&Product> = cat.search("rust").collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, 1);
}

#[test]
fn recommendations_return_related_products() {
    let mut cat = Catalogue::new();
    cat.insert_product(Product {
        id: 1,
        name: "Wireless Mouse".to_string(),
        category: "Electronics".to_string(),
        tags: vec!["mouse".to_string(), "wireless".to_string()],
    });
    cat.insert_product(Product {
        id: 2,
        name: "Gaming Keyboard".to_string(),
        category: "Electronics".to_string(),
        tags: vec!["keyboard".to_string(), "gaming".to_string()],
    });
    cat.insert_product(Product {
        id: 3,
        name: "Office Chair".to_string(),
        category: "Furniture".to_string(),
        tags: vec!["chair".to_string(), "office".to_string()],
    });
    cat.build_graph();
    let recs = cat.recommend(1, 1);

    assert!(recs.iter().any(|p| p.id == 2));
    assert!(!recs.iter().any(|p| p.id == 3));
}