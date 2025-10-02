use clap::Parser;
use megastore_search::{Catalogue, Product};
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(name = "megastore_search")]
struct Args {
    #[arg(long)]
    query: Option<String>,
    #[arg(long, default_value_t = 1)]
    id: usize,
    #[arg(long, default_value_t = 1)]
    depth: usize,
    #[arg(long)]
    input: Option<String>,
}

#[derive(Deserialize)]
struct ProductIn {
    id: usize,
    name: String,
    category: String,
    tags: Vec<String>,
}

fn load_from_json(path: &str) -> anyhow::Result<Vec<Product>> {
    let data = fs::read_to_string(path)?;
    let items: Vec<ProductIn> = serde_json::from_str(&data)?;
    Ok(items
        .into_iter()
        .map(|p| Product {
            id: p.id,
            name: p.name,
            category: p.category,
            tags: p.tags,
        })
        .collect())
}

fn default_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Smartphone X".to_string(),
            category: "Electronics".to_string(),
            tags: vec!["phone".to_string(), "camera".to_string()],
        },
        Product {
            id: 2,
            name: "Laptop Pro 15".to_string(),
            category: "Computers".to_string(),
            tags: vec!["laptop".to_string(), "pro".to_string()],
        },
        Product {
            id: 3,
            name: "USB-C Charger".to_string(),
            category: "Accessories".to_string(),
            tags: vec!["charger".to_string(), "usb-c".to_string()],
        },
        Product {
            id: 4,
            name: "Wireless Earbuds".to_string(),
            category: "Audio".to_string(),
            tags: vec!["earbuds".to_string(), "wireless".to_string()],
        },
    ]
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let products = if let Some(path) = args.input.as_deref() {
        load_from_json(path)?
    } else {
        default_products()
    };

    let mut cat = Catalogue::new();
    for p in products {
        cat.insert_product(p);
    }
    cat.build_graph();

    if let Some(q) = args.query.as_deref() {
        let results: Vec<_> = cat.search(q).collect();
        println!("Resultados da busca para '{}':", q);
        for p in results {
            println!("- {} (id: {})", p.name, p.id);
        }
    }

    let recs = cat.recommend(args.id, args.depth);
    println!("\nRecomendações para o produto id {}:", args.id);
    for p in recs {
        println!("- {} (id: {})", p.name, p.id);
    }

    Ok(())
}
