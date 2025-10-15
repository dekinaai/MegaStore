mod data;
mod search;
mod graph;
use crate::search::SearchIndex;
use crate::data::load_sample_products;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let products = load_sample_products()?;
    let mut idx = SearchIndex::new();
    for p in products.clone() {
        idx.add_product(p);
    }
    idx.link_products(1,2);
    idx.link_products(1,6);
    idx.link_products(3,9);
    idx.link_products(8,3);

    println!("MegaStore - Demo de Busca e Recomendação\n");

    loop {
        println!("Digite um comando: (search <query> | recommend <product_id> <n> | exit)");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "exit" { break; }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "search" => {
                let q = parts[1..].join(" ");
                let res = idx.search(&q);
                println!("Resultados ({}):", res.len());
                for p in res.iter().take(20) {
                    println!("- {} | {} | {} (id={})", p.name, p.brand, p.category, p.id);
                }
            },
            "recommend" => {
                if parts.len() < 3 {
                    println!("usage: recommend <product_id> <n>");
                    continue;
                }
                let pid: usize = parts[1].parse().unwrap_or(0);
                let n: usize = parts[2].parse().unwrap_or(5);
                let recs = idx.recommend(pid, n);
                println!("Recomendações para {}:", pid);
                for p in recs {
                    println!("- {} | {} | {} (id={})", p.name, p.brand, p.category, p.id);
                }
            },
            _ => println!("Comando desconhecido"),
        }
    }

    Ok(())
}
