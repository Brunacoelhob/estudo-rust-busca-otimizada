use std::collections::HashMap;
use crate::models::product::Produto;

pub struct IndiceDeBusca {
    pub indice: HashMap<String, Vec<Produto>>,
}

impl IndiceDeBusca {
    pub fn new() -> Self {
        IndiceDeBusca {
            indice: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, termo: &str, produto: Produto) {
        self.indice
            .entry(termo.to_lowercase())
            .or_insert_with(Vec::new)
            .push(produto);
    }

    pub fn buscar(&self, termo: &str) -> Option<&Vec<Produto>> {
        self.indice.get(&termo.to_lowercase())
    }
}
