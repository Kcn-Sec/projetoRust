    pub struct Usuario {
        pub nome:String,
        pub uid: u16,
        pub grupos_ids: Vec<u16>,
    }

    impl Usuario{
        pub fn new(nome: String, uid: u16) -> Usuario{
            Usuario{
                nome,
                uid,
                grupos_ids: Vec::new(),
            }
        }

        //add grupos pros usuarios
        pub fn add_grupo(&mut self, gid: u16){
            self.grupos_ids.push(gid);
        }

        //list
        pub fn listar_gid(&self){
            println!("Usuario: {}", self.nome);
            println!("UID: {}", self.uid);
            println!("Grupos:");

            for grupo in &self.grupos_ids{
                println!("GID: {}", grupo);
            }
        }
    }