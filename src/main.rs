//! Demonstração: `cargo run -- "note gam"` busca no catálogo de exemplo.

use busca_otimizada::{IndiceDeBusca, Produto};

fn catalogo_de_exemplo() -> IndiceDeBusca {
    let mut indice = IndiceDeBusca::new();
    let itens = [
        (1, "Notebook Gamer", "TechTop", "Informática"),
        (2, "Smartphone X", "MarcaTech", "Eletrônicos"),
        (3, "Mouse Gamer", "TechTop", "Periféricos"),
        (4, "Teclado Mecânico", "Logi", "Periféricos"),
        (5, "Monitor 24 polegadas", "ViewMax", "Monitores"),
    ];
    for (id, nome, marca, categoria) in itens {
        indice.adicionar(Produto::novo(id, nome, marca, categoria));
    }
    indice
}

fn main() {
    let consulta = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if consulta.trim().is_empty() {
        eprintln!("uso: busca-otimizada <termos da busca>   (ex.: busca-otimizada note gam)");
        std::process::exit(2);
    }

    let indice = catalogo_de_exemplo();
    let resultados = indice.buscar_limitado(&consulta, 10);
    if resultados.is_empty() {
        println!("Nenhum produto encontrado para '{consulta}'.");
        return;
    }
    println!("{} resultado(s) para '{consulta}':", resultados.len());
    for p in resultados {
        println!(
            "  #{:<3} {:<22} {:<10} {}",
            p.id, p.nome, p.marca, p.categoria
        );
    }
}
