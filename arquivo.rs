// struct que representa o arquivo
pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    // Permissoes do dono, do grupo e dos outros
    pub permissoes: (bool, bool, bool),
    pub uid_dono: u16,
    pub gid_grupo: u16,
}

impl Arquivo {
    // Aqui vai a implementação de Arquivo::new
    pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
    }

    // Aqui vai a implementação de Arquivo::alterar_permissao
    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
    }

    // Aqui vai a implementação de Arquivo::stat
    pub fn stat(&self) {
    }
}