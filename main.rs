mod album;
mod artista;
mod assinante;
mod extracao;
mod midias;
mod plataforma_digital;
mod podcaster;
mod produtor;
mod usuario;
use std::collections::HashMap;
use plataforma_digital::PlataformaDigital;

fn main()
{
    let mut platform = PlataformaDigital{
        nome: "Juliane".to_string(), 
        hash_poc: HashMap::new(),
        hash_albuns: HashMap::new(),
        hash_assinante: HashMap::new(),
        hash_produtor: HashMap::new(),
        hash_musica: HashMap::new(),
        hash_podcast: HashMap::new(),
        hash_genero: HashMap::new(),
        hash_artista: HashMap::new(),
        hash_podcaster: HashMap::new(),
    };
    extracao::adiciona_generos(&mut platform);
    extracao::adiciona_usuarios(&mut platform);
    extracao::adiciona_midias(&mut platform);
}