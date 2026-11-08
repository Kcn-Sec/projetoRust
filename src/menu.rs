use std::io::{self, Write};
 
use crate::permissao::Permissao;
use crate::arquivo::Arquivo;
use crate::diretorio::Diretorio;
use crate::grupo::Grupo;
use crate::usuario::Usuario;
 
// -------------------------------input do teclado
 
// mostra a mensagem e le uma linha digitada
fn ler_texto(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap(); //flush faz a mensagem aparecer antes da entrada, vou passar 2 videos pra voce nicolas sobre isso
 
    let mut entrada = String::new();
    let bytes = io::stdin().read_line(&mut entrada).expect("Erro de leitura");
    if bytes == 0 {
        std::process::exit(0); // fim da entrada 
    }
    entrada.trim().to_string()
}
 
// le um numero; se digitar algo invalido, vira 0
fn ler_numero(mensagem: &str) -> u64 {
    ler_texto(mensagem).parse().unwrap_or(0)
}
 
// le um numero de 0 a 7 e transforma em Permissao (4 = leitura, 2 = escrita, 1 = execucao)
fn ler_permissao(quem: &str) -> Permissao {
    let n = ler_numero(&format!("Permissao do {} (0 a 7): ", quem));
    Permissao::new((n & 4) != 0, (n & 2) != 0, (n & 1) != 0)
}
 // ainda nao entendi direito

// menu de criar / editar / apagar / listar, igual para quase todas as estruturas
fn submenu(titulo: &str) -> u64 {
    println!("{}", titulo);
    println!("1. criar");
    println!("2. editar");
    println!("3. apagar");
    println!("4. listar");
    println!("5. voltar");
    ler_numero("Opcao: ")
}
 
// ------------------------usuarios
 
fn gerenciar_usuarios(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    loop {
        match submenu("USUARIOS") {
            1 => {
                let nome = ler_texto("Nome: ");
                let uid = ler_numero("UID: ") as u16;
                if usuarios.iter().any(|u| u.uid == uid) {
                    println!("Ja existe usuario com esse UID.");
                    continue;
                }
                usuarios.push(Usuario::new(nome, uid));
                println!("Usuario criado!");
            }
            2 => {
                let uid = ler_numero("UID do usuario: ") as u16;
                let i = match usuarios.iter().position(|u| u.uid == uid) {
                    Some(i) => i,
                    None => { println!("Usuario nao encontrado."); continue; }
                };
 
                println!("1. alterar nome");
                println!("2. adicionar a um grupo");
                let opcao = ler_numero("Opcao: ");
 
                if opcao == 1 {
                    usuarios[i].nome = ler_texto("Novo nome: ");
                    println!("Nome alterado!");
                } else if opcao == 2 {
                    let gid = ler_numero("GID do grupo: ") as u16;
                    match grupos.iter().position(|g| g.gid == gid) {
                        Some(j) => {
                            usuarios[i].add_grupo(gid);
                            grupos[j].adiciona_membro(uid);
                            println!("Usuario adicionado ao grupo!");
                        }
                        None => println!("Grupo nao encontrado."),
                    }
                } else {
                    println!("Opcao invalida!");
                }
            }
            3 => {
                let uid = ler_numero("UID do usuario: ") as u16;
                match usuarios.iter().position(|u| u.uid == uid) {
                    Some(i) => {
                        usuarios.remove(i);
                        // tira o usuario da lista de membros dos grupos
                        for g in grupos.iter_mut() {
                            g.membros_uids.retain(|&u| u != uid);
                        }
                        println!("usuario deletado");
                    }
                    None => println!("usuario nao foi encontrado..."),
                }
            }
            4 => {
                for u in usuarios.iter() {
                    println!("------");
                    u.listar_gid();
                }
            }
            5 => break,
            _ => println!("Opcao invalida!"),
        }
    }
}
 
// -----------------grupos
 
fn gerenciar_grupos(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    loop {
        match submenu("GRUPOS") {
            1 => {
                let nome = ler_texto("Nome: ");
                let gid = ler_numero("GID: ") as u16;
                if grupos.iter().any(|g| g.gid == gid) {
                    println!("ja existe grupo com esse GID.");
                    continue;
                }
                grupos.push(Grupo::new(nome, gid));
                println!("Grupo criado!");
            }
            2 => {
                let gid = ler_numero("GID do grupo: ") as u16;
                let j = match grupos.iter().position(|g| g.gid == gid) {
                    Some(j) => j,
                    None => { println!("Grupo nao encontrado."); continue; }
                };
 
                println!("1. alterar nome");
                println!("2. adicionar membro");
                let opcao = ler_numero("Opcao: ");
 
                if opcao == 1 {
                    grupos[j].nome = ler_texto("Novo nome: ");
                    println!("Nome alterado!");
                } else if opcao == 2 {
                    let uid = ler_numero("UID do usuario: ") as u16;
                    match usuarios.iter().position(|u| u.uid == uid) {
                        Some(i) => {
                            grupos[j].adiciona_membro(uid);
                            usuarios[i].add_grupo(gid);
                            println!("Membro adicionado!");
                        }
                        None => println!("Usuario nao encontrado."),
                    }
                } else {
                    println!("Opcao invalida!");
                }
            }
            3 => {
                let gid = ler_numero("GID do grupo: ") as u16;
                match grupos.iter().position(|g| g.gid == gid) {
                    Some(j) => {
                        grupos.remove(j);
                        // tira o grupo da lista de grupos dos usuarios
                        for u in usuarios.iter_mut() {
                            u.grupos_ids.retain(|&g| g != gid);
                        }
                        println!("grupo apagado!");
                    }
                    None => println!("grupo nao encontrado"),
                }
            }
            4 => {
                for g in grupos.iter() {
                    println!("-------------");
                    g.listar_membro();
                }
            }
            5 => break,
            _ => println!("Opcao invalida!"),
        }
    }
}
 
// ------------arquivos
 
fn gerenciar_arquivos(arquivos: &mut Vec<Arquivo>) {
    loop {
        match submenu("ARQUIVOS") {
            1 => {
                let nome = ler_texto("Nome: ");
                let tamanho = ler_numero("Tamanho (em bites): ");
                let uid = ler_numero("UID do dono: ") as u16;
                let gid = ler_numero("GID do grupo: ") as u16;
                arquivos.push(Arquivo::new(nome, tamanho, uid, gid));
                println!("Arquivo criado!");
            }
            2 => {
                let nome = ler_texto("Nome do arquivo: ");
                let i = match arquivos.iter().position(|a| a.nome == nome) {
                    Some(i) => i,
                    None => { println!("Arquivo nao encontrado."); continue; }
                };
 
                println!("1. alterar nome");
                println!("2. alterar tamanho");
                let opcao = ler_numero("Opcao: ");
 
                if opcao == 1 {
                    arquivos[i].nome = ler_texto("Novo nome: ");
                    println!("Nome alterado!");
                } else if opcao == 2 {
                    arquivos[i].tamanho = ler_numero("Novo tamanho (bytes): ");
                    println!("Tamanho alterado!");
                } else {
                    println!("Opcao invalida!");
                }
            }
            3 => {
                let nome = ler_texto("Nome do arquivo: ");
                match arquivos.iter().position(|a| a.nome == nome) {
                    Some(i) => {
                        arquivos.remove(i);
                        println!("Arquivo apagado!");
                    }
                    None => println!("Arquivo nao encontrado."),
                }
            }
            4 => {
                for a in arquivos.iter() {
                    println!("-----------------");
                    a.stat();
                }
            }
            5 => break,
            _ => println!("Opcao invalida!"),
        }
    }
}
 
// ------------ diretorios // revisar o match
 
fn gerenciar_diretorios(diretorios: &mut Vec<Diretorio>, arquivos: &mut Vec<Arquivo>) {
    loop {
        match submenu("DIRETORIOS") {
            1 => {
                let nome = ler_texto("Nome: ");
                let uid = ler_numero("UID do dono: ") as u16;
                let permissoes = (
                    ler_permissao("dono"),
                    ler_permissao("grupo"),
                    ler_permissao("outros"),
                );
                diretorios.push(Diretorio::new(nome, permissoes, uid));
                println!("Diretorio criado!");
            }
            2 => {
                let nome = ler_texto("Nome do diretorio: ");
                let i = match diretorios.iter().position(|d| d.nome == nome) {
                    Some(i) => i,
                    None => { println!("Diretorio nao encontrado."); continue; }
                };
 
                println!("1. alterar nome");
                println!("2. adicionar arquivo");
                let opcao = ler_numero("Opcao: ");
 
                if opcao == 1 {
                    diretorios[i].nome = ler_texto("Novo nome: ");
                    println!("Nome alterado!");
                } else if opcao == 2 {
                    let nome_arq = ler_texto("Nome do arquivo: ");
                    match arquivos.iter().position(|a| a.nome == nome_arq) {
                        Some(k) => {
                            // tira o arquivo da lista geral e coloca dentro do diretorio
                            let arquivo = arquivos.remove(k);
                            diretorios[i].add_arquivo(arquivo);
                            println!("Arquivo adicionado ao diretorio!");
                        }
                        None => println!("Arquivo nao encontrado."),
                    }
                } else {
                    println!("Opcao invalida!");
                }
            }
            3 => {
                let nome = ler_texto("Nome do diretorio: ");
                match diretorios.iter().position(|d| d.nome == nome) {
                    Some(i) => {
                        diretorios.remove(i);
                        println!("Diretorio apagado!");
                    }
                    None => println!("Diretorio nao encontrado."),
                }
            }
            4 => {
                for d in diretorios.iter() {
                    println!("-----------------");
                    d.listar();
                    println!("UID do dono: {}", d.uid_dono);
                }
            }
            5 => break,
            _ => println!("Opcao invalida!"),
        }
    }
}
 
// ----------------permissoes
 
fn gerenciar_permissoes(arquivos: &mut Vec<Arquivo>, diretorios: &mut Vec<Diretorio>) {
    loop {
        println!("\n--- PERMISSOES ---");
        println!("1. alterar permissoes de um arquivo");
        println!("2. alterar permissoes de um diretorio");
        println!("3. listar permissoes");
        println!("4. voltar");
 
        match ler_numero("Opcao: ") {
            1 => {
                let nome = ler_texto("Nome do arquivo: ");
                let i = match arquivos.iter().position(|a| a.nome == nome) {
                    Some(i) => i,
                    None => { println!("Arquivo nao encontrado."); continue; }
                };
                let p = (ler_permissao("dono"), ler_permissao("grupo"), ler_permissao("outros"));
                arquivos[i].alterar_permissao(p);
                println!("Permissoes alteradas!");
            }
            2 => {
                let nome = ler_texto("Nome do diretorio: ");
                let i = match diretorios.iter().position(|d| d.nome == nome) {
                    Some(i) => i,
                    None => { println!("Diretorio nao encontrado."); continue; }
                };
                diretorios[i].permissoes = (
                    ler_permissao("dono"),
                    ler_permissao("grupo"),
                    ler_permissao("outros"),
                );
                println!("Permissoes alteradas!");
            }
            3 => {
                println!("== Arquivos ==");
                for a in arquivos.iter() {
                    println!("{}: {:?}", a.nome, a.permissoes);
                }
                println!("== Diretorios ==");
                for d in diretorios.iter() {
                    println!(
                        "{}: {}{}{}",
                        d.nome,
                        d.permissoes.0.octal(),
                        d.permissoes.1.octal(),
                        d.permissoes.2.octal()
                    );
                }
            }
            4 => break,
            _ => println!("Opcao invalida!"),
        }
    }
}
 
// ----------main menu
// eh essa funcao que o main chama
 
pub fn executar(
    usuarios: &mut Vec<Usuario>,
    grupos: &mut Vec<Grupo>,
    arquivos: &mut Vec<Arquivo>,
    diretorios: &mut Vec<Diretorio>,
) {
    loop {
        println!("\n-- MAIN MENU --");
        println!("1. gerenciar usuarios");
        println!("2. gerenciar grupos");
        println!("3. gerenciar arquivos");
        println!("4. gerenciar diretorios");
        println!("5. gerenciar permissoes");
        println!("6. sair");
 
        match ler_numero("Insira o numero da opcao: ") {
            1 => gerenciar_usuarios(usuarios, grupos),
            2 => gerenciar_grupos(grupos, usuarios),
            3 => gerenciar_arquivos(arquivos),
            4 => gerenciar_diretorios(diretorios, arquivos),
            5 => gerenciar_permissoes(arquivos, diretorios),
            6 => {
                println!("encerrando...");
                break;
            }
            _ => println!("Opcao invalida!"),
        }
    }
}
 