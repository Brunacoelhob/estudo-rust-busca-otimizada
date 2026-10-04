//! Normalização e quebra de texto em termos de busca.

/// Troca letras acentuadas pela letra base (português e alguns usos comuns): "Eletrônicos" → "eletronicos".
fn sem_acento(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'å' => 'a',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'ñ' => 'n',
        outro => outro,
    }
}

/// Minúsculas e sem acentos.
pub fn normalizar(texto: &str) -> String {
    texto
        .chars()
        .flat_map(char::to_lowercase)
        .map(sem_acento)
        .collect()
}

/// Quebra o texto em termos: sequências de letras/dígitos, normalizadas.
/// "Smartphone X-2 (Pro)" → ["smartphone", "x", "2", "pro"].
pub fn termos(texto: &str) -> Vec<String> {
    normalizar(texto)
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn remove_acentos_e_maiusculas() {
        assert_eq!(
            normalizar("Eletrônicos AÇÃO Ñandú"),
            "eletronicos acao nandu"
        );
    }

    #[test]
    fn quebra_em_termos_ignorando_pontuacao() {
        assert_eq!(
            termos("Smartphone X-2 (Pro)"),
            vec!["smartphone", "x", "2", "pro"]
        );
    }

    #[test]
    fn texto_vazio_ou_so_pontuacao_nao_gera_termos() {
        assert!(termos("").is_empty());
        assert!(termos(" -- ,, ").is_empty());
    }
}
