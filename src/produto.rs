/// Item do catálogo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

impl Produto {
    pub fn novo(id: u32, nome: &str, marca: &str, categoria: &str) -> Self {
        Self {
            id,
            nome: nome.to_owned(),
            marca: marca.to_owned(),
            categoria: categoria.to_owned(),
        }
    }
}
