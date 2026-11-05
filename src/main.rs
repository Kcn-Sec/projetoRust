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

// Estrutura que representa um arquivo.
pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    // Permissoes do dono, do grupo e dos outros.
    pub permissoes: (Permissao, Permissao, Permissao),
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

// Estrutura que representa um diretorio.
pub struct Diretorio {
    pub nome: String,
    pub arquivos: Vec<Arquivo>,
    // Permissoes do dono, do grupo e dos outros.
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
}

impl Diretorio {
    // Aqui vai a implementação de Diretorio::new
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
    }

    // Aqui vai a implementação de Diretorio::adiciona_arquivo
    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
    }

    // Aqui vai a implementação de Diretorio::remove_arquivo
    pub fn remove_arquivo(&mut self, nome_arquivo: String) {
    }

    // Aqui vai a implementação de Diretorio::listar_conteudo
    pub fn listar_conteudo(&self) {
    }
}

// Estrutura que representa um usuario.
pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    // Vetor armazenando os GIDs dos grupos do usuario.
    pub grupos_ids: Vec<u16>,
}

impl Usuario {
    // Aqui vai a implementação de Usuario::new
    pub fn new(nome: String, uid: u16) -> Usuario {
    }

    // Aqui vai a implementação de Usuario::adiciona_grupo
    pub fn adiciona_grupo(&mut self, gid: u16) {
    }

    // Aqui vai a implementação de Usuario::remove_grupo
    pub fn remove_grupo(&mut self, gid: u16) {
    }

    // Aqui vai a implementação de Usuario::listar_grupos
    pub fn listar_grupos(&self) {
    }
}

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

// Aqui vai a implementação do menu interativo via terminal
fn exibir_menu() {}

fn main() {
    // Aqui vai a implementação da chamada do menu interativo
    exibir_menu();
}
