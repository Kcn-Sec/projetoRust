// Estrutura que representa um grupo.
pub struct Grupo {
    pub nome: String,
    pub gid: u16,
    // Vetor armazenando os UIDs dos membros do grupo.
    pub membros_uids: Vec<u16>,
}

impl Grupo {
    // Aqui vai a implementação de Grupo::new
    pub fn new(nome: String, gid: u16) -> Grupo {
    }

    // Aqui vai a implementação de Grupo::adiciona_membro
    pub fn adiciona_membro(&mut self, uid: u16) {
    }

    // Aqui vai a implementação de Grupo::remove_membro
    pub fn remove_membro(&mut self, uid: u16) {
    }

    // Aqui vai a implementação de Grupo::listar_membros
    pub fn listar_membros(&self) {
    }
}