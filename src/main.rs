use clap::Parser;
use megastore_search::prelude::*;
use std::fs;

#[derive(Parser)]
#[command(name = "megastore_search")]
struct Args {
    #[arg(long)]
    query: Option<String>,
    #[arg(long)]
    input: Option<String>,
    #[arg(long, default_value_t = 1)]
    id: usize,
    #[arg(long, default_value_t = 5)]
    k: usize
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut cat = Catalogue::new();
    if let Some(path) = args.input {
        let data = fs::read_to_string(path)?;
        let products: Vec<Product> = serde_json::from_str(&data)?;
        for p in products { cat.insert_product(p); }
    } else {
        cat.insert_product(Product{ id:1, name:"Rust Book".into(), category:"Books".into(), tags:vec!["programming".into()] });
        cat.insert_product(Product{ id:2, name:"Cooking for Beginners".into(), category:"Books".into(), tags:vec!["cooking".into()] });
        cat.insert_product(Product{ id:3, name:"Mechanical Keyboard".into(), category:"Electronics".into(), tags:vec!["keyboard".into(),"gaming".into()] });
        cat.insert_product(Product{ id:4, name:"Gaming Mouse".into(), category:"Electronics".into(), tags:vec!["mouse".into(),"gaming".into()] });
        cat.insert_product(Product{ id:5, name:"Office Chair".into(), category:"Furniture".into(), tags:vec!["chair".into(),"office".into()] });
    }
    cat.build_index();
    cat.build_graph();
    if let Some(q) = args.query {
        let results = cat.search(&q);
        println!("Resultados da busca:");
        for p in results { println!("{} {}", p.id, p.name); }
    }
    let recs = cat.recommend(args.id, args.k);
    println!("Recomendações:");
    for p in recs { println!("{} {}", p.id, p.name); }
    Ok(())
}