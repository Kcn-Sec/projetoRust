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
use std::io;

fn ler() -> String {
    let mut texto = String::new();
    io::stdin().read_line(&mut texto).unwrap();
    texto.trim().to_string()
}

// impl menu terminal
fn exibir_menu() {
    println!("Menu:");
    println!("1 - Criar usuário");
    println!("2 - Listar Usuários");
    println!("3 - Apagar Usuário");
    println!("0 - Sair");
    println!("Opção:");
}

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    // impl do menu
    loop{ 
    exibir_menu();
    let opcao = ler();
        match opcao.as_str(){
        "1" => {
            println!("Nome:");
            let nome = ler();
            println!("UID:");
            let uid: u16 = ler().parse().unwrap();
            usuarios.push(Usuario::new(nome, uid));
            println!("Usuario criado!");
        }
        "2" => {
            for u in &usuarios {
                println!("{} (uid {})", u.nome, u.uid);
            }
        }
        "3" => {
            println!("UID do usuario a apagar:");
            let uid: u16 = ler().parse().unwrap();
            usuarios.retain(|u| u.uid != uid);
            println!("Usuario apagado");
        }
            "0" => break,
            _ => println!("Opção inválida"),
        }
    }

}
