//mod midias;
mod album;
mod artista;
mod assinante;
mod extracao;
mod midias;
mod plataformaDigital;
mod podcaster;
mod produtor;
mod usuario;
mod escrita;
use std::collections::HashMap;
use plataformaDigital::PlataformaDigital;


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
    for(sigla, genero) in platform.hash_podcast.iter()
    {
        println!("O código é {} e o significado é {}", sigla, genero.nome);
    }
    extracao::adiciona_favoritos(&mut platform);
    for(id, favoritos) in platform.hash_assinante.iter()
    {
        println!("Os favoritos do usuario {} são {:?}", id, favoritos.lista_favoritos)
    }
    escrita::midias_por_produtor(&platform);
    escrita::faz_backup_usuarios(&platform);
}