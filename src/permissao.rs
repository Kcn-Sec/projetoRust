// Estrutura que representa as permissoes de leitura, escrita e execucao.
pub struct Permissao {
    pub leitura: bool,
    pub escrita: bool,
    pub execucao: bool,
}

impl Permissao {
    // Aqui vai a implementação de Permissao::new
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    // Aqui vai a implementação de Permissao::octal
    pub fn octal(&self) -> u8 {
        (self.leitura as u8) * 4 + (self.escrita as u8) * 2 + (self.execucao as u8) * 1
    }
}
