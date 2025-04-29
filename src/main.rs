use projeto::models::product::Produto;
use projeto::search::index::IndiceDeBusca;

fn main() {
    let mut indice = IndiceDeBusca::new();

    let produto = Produto {
        id: 1,
        nome: "Smartphone X".to_string(),
        marca: "MarcaTech".to_string(),
        categoria: "Eletrônicos".to_string(),
    };

    indice.adicionar(&produto.nome, produto.clone());

    println!("Busca por 'Smartphone X': {:?}", indice.buscar("Smartphone X"));
}
