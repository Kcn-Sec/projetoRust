use crate::arquivo::Arquivo;
use crate::permissao::Permissao;

pub struct Diretorio{
    pub nome: String,
    pub arquivos: Vec<Arquivo>,
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
}

// o constructor eh muito bom nesses casos porque ele cria um diretorio novo e vai preenchendo os atributos com os valores recebidos
impl Diretorio{
        pub fn new(
            nome: String,
            permissoes: (Permissao, Permissao, Permissao),
            uid_dono: u16
        ) -> Diretorio {
            Diretorio{
                nome,
                arquivos: Vec::new(), //vetor vazio pra armazenar os arquivos do diretorio
                permissoes,
                uid_dono,
            }
        }
    // adicionar arquivo
    pub fn add_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo); //push adiciona um arquivo novo no final do vetor
    }

    //listar
    pub fn listar(&self){
        println!("Diretorio: {}", self.nome);
        println!("Arquivos:");

        for arquivo in &self.arquivos{
            println!("{}", arquivo.nome);
        }
    }
}