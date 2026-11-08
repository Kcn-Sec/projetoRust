use crate::permissao::Permissao;
// struct que representa o arquivo
pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    // Permissoes do dono, do grupo e dos outros
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
    pub gid_grupo: u16,
}

impl Arquivo {
    // Aqui vai a implementação de Arquivo::new
    pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, true, false),
                Permissao::new(true, true, false),
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    // Aqui vai a implementação de Arquivo::alterar_permissao
    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    // Aqui vai a implementação de Arquivo::stat
    pub fn stat(&self) {
        let (dono, grupo, outros) = &self.permissoes;
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!(
            "Permissoes: ({}{}{} / {}{}{})",
            dono.octal(),
            grupo.octal(),
            outros.octal(),
            dono.rwx(),
            grupo.rwx(),
            outros.rwx()
        );
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}
