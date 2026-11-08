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
    println!("4 - Criar grupo");
    println!("5 - Listar grupos");
    println!("6 - Adicionar usuário a um grupo");
    println!("7 - Criar diretório");
    println!("8 - Listar diretórios");
    println!("9 - Criar arquivo em diretório");
    println!("10 - Ver stat de arquivo");
    println!("0 - Sair");
    println!("Opção:");
}

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();
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
        "4" => {
            println!("Nome do grupo:");
            let nome = ler();
            println!("GID:");
            let gid: u16 = ler().parse().unwrap();
            grupos.push(Grupo::new(nome, gid));
            println!("Grupo criado!");
        }
        "5" => {
            for g in &grupos {
                println!("{} (gid {})", g.nome, g.gid);
                g.listar_membros();
            }
        }
        "6" => {
            println!("UID do usuário:");
            let uid: u16 = ler().parse().unwrap();
            println!("GID do grupo:");
            let gid: u16 = ler().parse().unwrap();
            
            let usuario = usuarios.iter_mut().find(|u| u.uid == uid);
            let grupo = grupos.iter_mut().find(|g| g.gid == gid);

            match (usuario, grupo) {
                (Some(u), Some(g)) => {
                    u.adiciona_grupo(gid);
                    g.adiciona_membro(uid);
                    println!("Usuário adicionado ao grupo!");
                }
                _ => println!("Usuário ou grupo não encontrado"),
            }
        }
        "7" => {
            println!("Nome do diretório:");
            let nome = ler();
            println!("UID do dono:");
            let uid: u16 = ler().parse().unwrap();
            let perms = (
                Permissao::new(true, true, true),
                Permissao::new(true, false, true),
                Permissao::new(true, false, true),
                );
                diretorios.push(Diretorio::new(nome, perms, uid));
                println!("Diretorio criado!");
        }
        "8" => {
            for d in &diretorios {
                println!("Diretorio: {}", d.nome);
                d.listar_conteudo();
            }
        }
        "9" => {
            println!("Nome do diretório:");
            let dir = ler();
            match diretorios.iter_mut().find(|d| d.nome == dir) {
                Some(d) => {
                    println!("Nome do arquivo:");
                    let nome = ler();
                    println!("Tamanho em bytes:");
                    let tamanho: u64 = ler().parse().unwrap();
                    println!("UID do dono:");
                    let uid: u16 = ler().parse().unwrap();
                    println!("GID do grupo:");
                    let gid: u16 = ler().parse().unwrap();
                    d.adiciona_arquivo(Arquivo::new(nome, tamanho, uid, gid));
                    println!("Arquivo criado!");
                }
                None => println!("Diretório não encontrado!"),
            }
        }
        "10" => {
            println!("Nome do diretório:");
            let dir = ler();
            println!("Nome do arquivo:");
            let arquivo = ler();
            match diretorios.iter().find(|d| d.nome == dir) {
                Some(d) => match d.arquivos.iter().find(|a| a.nome == arquivo) {
                    Some(a) => a.stat(),
                    None => println!("Arquivo não encontrado!"),
                },
                None => println!("Diretório não encontrado!"),
            }
        }
            "0" => break,
            _ => println!("Opção inválida"),
        }
    }

}
