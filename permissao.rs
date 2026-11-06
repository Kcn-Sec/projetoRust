// Estrutura que representa as permissoes de leitura, escrita e execucao.
pub struct Permissao {
    pub leitura: bool,
    pub escrita: bool,
    pub execucao: bool,
}

impl Permissao {
    // Aqui vai a implementação de Permissao::new
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
    }

    // Aqui vai a implementação de Permissao::octal
    pub fn octal(&self) -> u8 {
    }
}