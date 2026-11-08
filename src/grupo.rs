//estrutura grupo
pub struct Grupo{
    pub nome: String,
    pub gid: u16,
    pub membros_uids: Vec<u16>, //esse eh o vetor que armazena as uids 
}

    //construtor
    impl Grupo {
    pub fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }

    //adicionar membro
    pub fn adiciona_membro(&mut self, uid: u16){
        self.membros_uids.push(uid);
    }

    //listar 
    pub fn listar_membro(&self){
        println!("Grupo: {}", self.nome);
        println!("GID: {}", self.gid);
        println!("Membros:");

        for uid in &self.membros_uids {
            println!("UID {}", uid);
        }
    }
}