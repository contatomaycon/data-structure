use megastore_search::prelude::*;

#[test]
fn search_and_recommend() {
    let mut cat = Catalogue::new();
    cat.insert_product(Product{ id:1, name:"Rust Book".into(), category:"Books".into(), tags:vec!["programming".into()] });
    cat.insert_product(Product{ id:2, name:"Advanced Rust".into(), category:"Books".into(), tags:vec!["programming".into()] });
    cat.insert_product(Product{ id:3, name:"Mechanical Keyboard".into(), category:"Electronics".into(), tags:vec!["keyboard".into(),"gaming".into()] });
    cat.insert_product(Product{ id:4, name:"Gaming Mouse".into(), category:"Electronics".into(), tags:vec!["mouse".into(),"gaming".into()] });
    cat.insert_product(Product{ id:5, name:"Office Chair".into(), category:"Furniture".into(), tags:vec!["chair".into(),"office".into()] });
    cat.build_index();
    cat.build_graph();
    let results = cat.search("rust programming");
    assert!(results.iter().any(|p| p.id == 1));
    assert!(results.iter().any(|p| p.id == 2));
    let recs = cat.recommend(3, 2);
    assert!(recs.iter().any(|p| p.id == 4));
    assert!(!recs.iter().any(|p| p.id == 5));
}