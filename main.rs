// declaracao dos modulos
mod arquivo;
mod diretorio;
mod grupo;
mod permissao;
mod usuario;

// usando varios pra nao precisar usar prefixo depois
use arquivo::Arquivo;
use diretorio::Diretorio;
use grupo::Grupo;
use permissao::Permissao;
use usuario::Usuario;



// impl menu terminal
fn exibir_menu() {}

fn main() {
    // impl do menu
    exibir_menu();
    println!("Testando git carai");    
}
