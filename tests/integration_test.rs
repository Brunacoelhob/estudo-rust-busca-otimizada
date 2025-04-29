use projeto::models::product::Produto;
use projeto::search::index::IndiceDeBusca;

#[test]
fn deve_adicionar_e_buscar_produto() {
    let mut indice = IndiceDeBusca::new();

    let produto = Produto {
        id: 1,
        nome: "Notebook Gamer".to_string(),
        marca: "TechTop".to_string(),
        categoria: "Informática".to_string(),
    };

    indice.adicionar(&produto.nome, produto.clone());

    let resultado = indice.buscar("Notebook Gamer");

    assert!(resultado.is_some());
    assert_eq!(resultado.unwrap().len(), 1);
    assert_eq!(resultado.unwrap()[0].id, 1);
}
