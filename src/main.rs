mod permissao;
mod arquivo;
mod diretorio;
mod grupo;
mod usuario;
mod menu;
 
use arquivo::Arquivo;
use diretorio::Diretorio;
use grupo::Grupo;
use usuario::Usuario;
 
fn main(){
    // os vetores guardam tudo que for criado pelo menu
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();
 
    menu::executar(&mut usuarios, &mut grupos, &mut arquivos, &mut diretorios);
}
 
