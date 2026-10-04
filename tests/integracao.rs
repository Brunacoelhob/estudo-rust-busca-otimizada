use busca_otimizada::{IndiceDeBusca, Produto};

#[test]
fn fluxo_completo_adicionar_buscar_atualizar_remover() {
    let mut indice = IndiceDeBusca::new();
    indice.adicionar(Produto::novo(1, "Notebook Gamer", "TechTop", "Informática"));

    // O teste original só achava digitando o nome EXATO; agora basta parte dele.
    let achados = indice.buscar("Notebook Gamer");
    assert_eq!(achados.len(), 1);
    assert_eq!(achados[0].id, 1);
    assert_eq!(indice.buscar("note gam").len(), 1);

    indice.adicionar(Produto::novo(
        1,
        "Notebook Office",
        "TechTop",
        "Informática",
    ));
    assert!(indice.buscar("gamer").is_empty());
    assert_eq!(indice.buscar("office").len(), 1);

    assert!(indice.remover(1).is_some());
    assert!(indice.is_empty());
    assert!(indice.buscar("notebook").is_empty());
}
