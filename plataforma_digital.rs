use std::collections::HashMap;
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