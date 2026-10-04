//! Busca eficiente em catálogo de produtos com índice invertido, sem dependências externas.
//!
//! ```
//! use busca_otimizada::{IndiceDeBusca, Produto};
//!
//! let mut indice = IndiceDeBusca::new();
//! indice.adicionar(Produto::novo(1, "Notebook Gamer", "TechTop", "Informática"));
//!
//! let achados = indice.buscar("note gam"); // prefixos, sem acento/maiúscula
//! assert_eq!(achados[0].id, 1);
//! ```

pub mod indice;
pub mod produto;
pub mod texto;

pub use indice::IndiceDeBusca;
pub use produto::Produto;
