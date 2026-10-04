//! Índice invertido para busca em catálogo.
//!
//! Em vez de uma busca exata por um texto inteiro (como no `HashMap<String, Vec<Produto>>` original,
//! que só achava "Notebook Gamer" digitando exatamente isso), cada produto é quebrado em termos
//! (nome, marca e categoria) e cada termo aponta para os produtos que o contêm.
//!
//! Complexidade: inserir/remover O(t · log n) (t = termos do produto, n = termos distintos);
//! buscar um prefixo O(log n + k) graças ao `BTreeMap` ordenado (k = termos que casam).

use std::collections::{BTreeMap, HashMap};

use crate::produto::Produto;
use crate::texto::termos;

const PESO_NOME: u32 = 3;
const PESO_MARCA: u32 = 2;
const PESO_CATEGORIA: u32 = 1;

#[derive(Debug, Default)]
pub struct IndiceDeBusca {
    produtos: HashMap<u32, Produto>,
    /// termo → (id do produto → peso do campo onde o termo apareceu)
    invertido: BTreeMap<String, HashMap<u32, u32>>,
}

impl IndiceDeBusca {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.produtos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.produtos.is_empty()
    }

    pub fn obter(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    /// Adiciona o produto. Se já existir um com o mesmo `id`, ele é **substituído** (e o antigo é devolvido),
    /// sem deixar entradas obsoletas no índice.
    pub fn adicionar(&mut self, produto: Produto) -> Option<Produto> {
        let anterior = self.remover(produto.id);
        for (texto, peso) in [
            (&produto.nome, PESO_NOME),
            (&produto.marca, PESO_MARCA),
            (&produto.categoria, PESO_CATEGORIA),
        ] {
            for termo in termos(texto) {
                let pesos = self.invertido.entry(termo).or_default();
                let atual = pesos.entry(produto.id).or_insert(0);
                *atual = (*atual).max(peso); // vale o campo mais importante em que o termo aparece
            }
        }
        self.produtos.insert(produto.id, produto);
        anterior
    }

    /// Remove o produto e limpa os termos que ficaram sem nenhum produto.
    pub fn remover(&mut self, id: u32) -> Option<Produto> {
        let produto = self.produtos.remove(&id)?;
        let mut candidatos: Vec<String> = [&produto.nome, &produto.marca, &produto.categoria]
            .into_iter()
            .flat_map(|t| termos(t))
            .collect();
        candidatos.sort_unstable();
        candidatos.dedup();
        for termo in candidatos {
            if let Some(pesos) = self.invertido.get_mut(&termo) {
                pesos.remove(&id);
                if pesos.is_empty() {
                    self.invertido.remove(&termo);
                }
            }
        }
        Some(produto)
    }

    /// Busca por **todos** os termos da consulta (E lógico), cada um casando como prefixo,
    /// sem diferenciar maiúsculas/acentos. Ex.: "note gam" acha "Notebook Gamer".
    /// O resultado vem ordenado por relevância (nome > marca > categoria) e, em empate, por id.
    /// Consulta vazia devolve lista vazia.
    pub fn buscar(&self, consulta: &str) -> Vec<&Produto> {
        self.buscar_limitado(consulta, usize::MAX)
    }

    /// Igual a [`buscar`](Self::buscar), mas devolve no máximo `limite` resultados.
    pub fn buscar_limitado(&self, consulta: &str, limite: usize) -> Vec<&Produto> {
        let termos_consulta = termos(consulta);
        if termos_consulta.is_empty() || limite == 0 {
            return Vec::new();
        }

        // id → pontuação acumulada; começa com o primeiro termo e vai interseccionando.
        let mut placar: Option<HashMap<u32, u32>> = None;
        for termo in &termos_consulta {
            let do_termo = self.ids_com_prefixo(termo);
            placar = Some(match placar {
                None => do_termo,
                Some(anterior) => anterior
                    .into_iter()
                    .filter_map(|(id, pontos)| do_termo.get(&id).map(|p| (id, pontos + p)))
                    .collect(),
            });
            if placar.as_ref().is_some_and(HashMap::is_empty) {
                return Vec::new(); // algum termo não casa com nada: não há resultado possível
            }
        }

        let mut ordenados: Vec<(u32, u32)> = placar.unwrap_or_default().into_iter().collect();
        ordenados.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        ordenados
            .into_iter()
            .take(limite)
            .filter_map(|(id, _)| self.produtos.get(&id))
            .collect()
    }

    /// Ids dos produtos que têm algum termo começando por `prefixo`, com o melhor peso de cada um.
    fn ids_com_prefixo(&self, prefixo: &str) -> HashMap<u32, u32> {
        let mut resultado: HashMap<u32, u32> = HashMap::new();
        for (termo, pesos) in self.invertido.range(prefixo.to_owned()..) {
            if !termo.starts_with(prefixo) {
                break; // o BTreeMap é ordenado: acabaram os termos com esse prefixo
            }
            for (&id, &peso) in pesos {
                let atual = resultado.entry(id).or_insert(0);
                *atual = (*atual).max(peso);
            }
        }
        resultado
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn catalogo() -> IndiceDeBusca {
        let mut i = IndiceDeBusca::new();
        i.adicionar(Produto::novo(1, "Notebook Gamer", "TechTop", "Informática"));
        i.adicionar(Produto::novo(2, "Smartphone X", "MarcaTech", "Eletrônicos"));
        i.adicionar(Produto::novo(3, "Mouse Gamer", "TechTop", "Periféricos"));
        i.adicionar(Produto::novo(4, "Teclado Mecânico", "Logi", "Periféricos"));
        i
    }

    fn ids(r: Vec<&Produto>) -> Vec<u32> {
        r.into_iter().map(|p| p.id).collect()
    }

    #[test]
    fn acha_por_termo_parcial_de_qualquer_campo() {
        let i = catalogo();
        assert_eq!(ids(i.buscar("gamer")), vec![1, 3]);
        assert_eq!(ids(i.buscar("note")), vec![1], "prefixo");
        assert_eq!(ids(i.buscar("periferico")), vec![3, 4], "categoria");
    }

    #[test]
    fn ignora_maiusculas_e_acentos_na_consulta_e_nos_dados() {
        let i = catalogo();
        assert_eq!(ids(i.buscar("MECÂNICO")), vec![4]);
        assert_eq!(ids(i.buscar("eletronicos")), vec![2]);
        assert_eq!(ids(i.buscar("informatica")), vec![1]);
    }

    #[test]
    fn varios_termos_exigem_todos() {
        let i = catalogo();
        assert_eq!(ids(i.buscar("gamer techtop")), vec![1, 3]);
        assert_eq!(ids(i.buscar("gamer notebook")), vec![1]);
        assert!(i.buscar("gamer smartphone").is_empty());
    }

    #[test]
    fn ordena_por_relevancia_nome_antes_de_marca_antes_de_categoria() {
        let mut i = IndiceDeBusca::new();
        i.adicionar(Produto::novo(1, "Cabo", "Redmi", "Acessórios")); // "redmi" na marca
        i.adicionar(Produto::novo(2, "Redmi Note", "Xiaomi", "Celulares")); // no nome
        i.adicionar(Produto::novo(3, "Capa", "Genérica", "Redmi")); // na categoria
        assert_eq!(ids(i.buscar("redmi")), vec![2, 1, 3]);
    }

    #[test]
    fn consulta_vazia_ou_sem_resultado() {
        let i = catalogo();
        assert!(i.buscar("").is_empty());
        assert!(i.buscar("   ").is_empty());
        assert!(i.buscar("---").is_empty());
        assert!(i.buscar("inexistente").is_empty());
    }

    #[test]
    fn respeita_o_limite_de_resultados() {
        let i = catalogo();
        assert_eq!(ids(i.buscar_limitado("gamer", 1)), vec![1]);
        assert!(i.buscar_limitado("gamer", 0).is_empty());
    }

    #[test]
    fn mesmo_id_substitui_sem_deixar_lixo_no_indice() {
        let mut i = catalogo();
        let antigo = i.adicionar(Produto::novo(
            1,
            "Monitor Ultrawide",
            "ViewMax",
            "Monitores",
        ));

        assert_eq!(antigo.unwrap().nome, "Notebook Gamer");
        assert_eq!(i.len(), 4);
        assert_eq!(
            ids(i.buscar("notebook")),
            Vec::<u32>::new(),
            "termos antigos sumiram"
        );
        assert_eq!(ids(i.buscar("ultrawide")), vec![1]);
    }

    #[test]
    fn remover_limpa_termos_orfaos() {
        let mut i = catalogo();
        assert!(i.remover(2).is_some());
        assert!(i.remover(2).is_none());
        assert!(i.buscar("smartphone").is_empty());
        assert!(
            !i.invertido.contains_key("smartphone"),
            "termo sem produtos é apagado"
        );
        assert!(
            i.invertido.contains_key("techtop"),
            "termo ainda usado é mantido"
        );
        assert_eq!(i.len(), 3);
    }

    #[test]
    fn funciona_com_catalogo_grande() {
        let mut i = IndiceDeBusca::new();
        for n in 0..20_000u32 {
            i.adicionar(Produto::novo(
                n,
                &format!("Produto{n} Especial"),
                "Marca",
                "Geral",
            ));
        }
        assert_eq!(i.len(), 20_000);
        assert_eq!(ids(i.buscar("produto19999")), vec![19_999]);
        assert_eq!(i.buscar("especial").len(), 20_000);
        assert_eq!(i.buscar_limitado("especial", 10).len(), 10);
    }
}
