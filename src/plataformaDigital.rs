use std::collections::HashMap;
use crate::album::Album;
use crate::artista::Artista;
use crate::usuario::ImprimeNoArquivo;
use crate::usuario;
use crate::artista;
use crate::album;
use crate::assinante;
use crate::produtor;
use crate::midias;
use crate::podcaster;


pub struct PlataformaDigital
{
    pub nome: String,
    pub hash_albuns: HashMap<String, album::Album>,
    pub hash_assinante: HashMap<String, assinante::Assinante>,
    pub hash_produtor: HashMap<String, produtor::Produtor>,
    pub hash_musica: HashMap<String, midias::Musica>,
    pub hash_podcast: HashMap<String, midias::Podcast>,
    pub hash_genero: HashMap<String, midias::Genero>,
    pub hash_artista: HashMap<String, artista::Artista>,
    pub hash_podcaster: HashMap<String, podcaster::Podcaster>,
    pub hash_poc: HashMap<String, String>,

    
}


// fn main()
// {
//     let mut platform = PlataformaDigital{
//         nome: "Juliane".to_string(), 
//         hash_poc: HashMap::new(),
//         hash_albuns: HashMap::new(),
//         hash_assinante: HashMap::new(),
//         hash_produtor: HashMap::new(),
//         hash_musica: HashMap::new(),
//         hash_podcast: HashMap::new(),
//         hash_genero: HashMap::new(),
//         hash_artista: HashMap::new(),
//         hash_podcaster: HashMap::new(),
//     };
//     platform.hash_poc.insert("Joana".to_string(), "99789-2714".to_string());
//     for (pessoa, numero) in platform.hash_poc.iter()
//     {
//         println!("A pessoa {} tem número {}", pessoa, numero)
//     }
//     platform.hash_albuns.insert("album".to_string(), Album::new("nome".to_string(), 10, 32.2, 2010, 8, "1".to_string()));
//     platform.hash_artista.insert("artista".to_string(), Artista::new("1".to_string(), vec!("10".to_string())));
//     for (nome, album) in platform.hash_albuns.iter()
//     {
//         album.imprime_no_arquivo("texte.txt".to_string());
//     }
//     for (nome, artista) in platform.hash_artista.iter()
//     {
//         artista.imprime_no_arquivo("outrotexte.txt".to_string(), platform.hash_musica[1]);
//     }
// }