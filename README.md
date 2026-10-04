# Busca Otimizada em Rust

[![CI](https://github.com/Brunacoelhob/estudo-rust-busca-otimizada/actions/workflows/ci.yml/badge.svg)](https://github.com/Brunacoelhob/estudo-rust-busca-otimizada/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

Busca em catálogo de produtos com **índice invertido**, sem dependências externas. Nasceu como solução para o problema de indexação e recuperação rápida de uma loja ("MegaStore") e evoluiu de um `HashMap` de busca exata para uma busca de verdade.

## O que ela faz

| Consulta | Acha |
|---|---|
| `note gam` | "Notebook Gamer" (prefixos, **todos** os termos precisam casar) |
| `ELETRÔNICOS` | produtos da categoria "Eletrônicos" (ignora maiúsculas e acentos) |
| `techtop` | produtos da marca TechTop (busca em nome, marca **e** categoria) |

Resultados ordenados por **relevância**: termo no nome (peso 3) > marca (2) > categoria (1); empate resolvido por id.

## Por que um índice invertido?

A versão original guardava `HashMap<texto completo → produtos>`: só achava "Notebook Gamer" se você digitasse exatamente "Notebook Gamer". Agora cada produto é quebrado em termos e cada termo aponta para os produtos que o contêm:

```
"notebook" → {1}        "gamer" → {1, 3}        "techtop" → {1, 3}
```

Um `BTreeMap` ordenado permite achar todos os termos de um prefixo em `O(log n + k)`. Inserir e remover são `O(t · log n)`, com limpeza de termos órfãos (nada de lixo ao atualizar um produto).

## Uso

```rust
use busca_otimizada::{IndiceDeBusca, Produto};

let mut indice = IndiceDeBusca::new();
indice.adicionar(Produto::novo(1, "Notebook Gamer", "TechTop", "Informática"));

let achados = indice.buscar("note gam");
assert_eq!(achados[0].id, 1);

indice.buscar_limitado("gamer", 10); // no máximo 10 resultados
indice.remover(1);
```

Adicionar um produto com `id` já existente o **substitui** (e devolve o antigo).

### Linha de comando

```bash
cargo run -- note gam
cargo run -- eletrônicos
```

## Desenvolvimento

Requer Rust 1.85+ (edição 2024).

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

14 testes cobrem relevância, acentos, múltiplos termos, limite, substituição, remoção sem lixo e um catálogo de 20 mil produtos.

## Estrutura

```
src/
├── lib.rs       # API pública (com doctest)
├── indice.rs    # índice invertido, busca e testes
├── produto.rs   # modelo de dados
├── texto.rs     # normalização e tokenização
└── main.rs      # demonstração em linha de comando
tests/integracao.rs
```

## Limitações

- O índice vive em memória (adequado para catálogos de dezenas/centenas de milhares de itens).
- A remoção de acentos cobre as letras do português e algumas comuns em outros idiomas latinos; não é uma normalização Unicode completa.
- Não há correção ortográfica nem busca por similaridade.

## Licença

[MIT](LICENSE)
