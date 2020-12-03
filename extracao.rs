extern crate csv;
use csv::ReaderBuilder;
use crate::midias;
use crate::plataforma_digital;
use crate::assinante::Assinante;
use crate::artista::Artista;
use crate::podcaster::Podcaster;
use crate::midias::{Musica, Podcast};
use crate::album::Album;


pub fn adiciona_generos(plataforma: &mut plataforma_digital::PlataformaDigital)
{
    let mut my_reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("entradas/generos.csv").expect("Problema no csv");

    for result in my_reader.records()
    {
        let record = result.expect("quero morrer.");
        let genero = midias::Genero::new(record[1].to_string(), record[0].to_string());
        plataforma.hash_genero.insert(record[0].to_string(), genero);
    }
}

pub fn adiciona_usuarios(plataforma: &mut plataforma_digital::PlataformaDigital)
{
    let mut my_reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("entradas/usuarios.csv").expect("Problema no csv");

    for result in my_reader.records()
    {
        let record = result.expect("bombom de amora");
        let codigo = &record[0];
        let tipo: String = record[1].to_string();
        let name = record[2].to_string();
       if tipo=="U"
       {
           let usuario = Assinante{user_id: codigo.to_string(), lista_favoritos: Vec::new(), user_name: name};
           plataforma.hash_assinante.insert(codigo.to_string(), usuario);

       }

       else if tipo == "A"
       {
           let artista = Artista{
               user_id: codigo.to_string(), lista_id_albuns: Vec::new(), 
               lista_musicas: Vec::new(), artist_name: name
            };
           plataforma.hash_artista.insert(codigo.to_string(), artista);
       }

       else if tipo == "P"
       {
           let podcaster = Podcaster{user_id: codigo.to_string(), lista_podcasts: Vec::new(), podcaster_name: name};
           plataforma.hash_podcaster.insert(codigo.to_string(), podcaster);
       }
    }
}

pub fn adiciona_midias(plataforma: &mut plataforma_digital::PlataformaDigital)
{
    let mut my_reader_semicolon = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("entradas/midias.csv").expect("Problema no csv");

    for result in my_reader_semicolon.records()
    {
        let record = result.expect("Trouble in paradise");
        let codigo_midia = record[0].to_string();
        let nome = record[1].to_string();
        let tipo = record[2].to_string();
        let artista_id = record[3].to_string();
        let duracao: f32 = record[4].replace(',', ".").parse().unwrap();
        let generos = record[5].split(", ");
        let temporada = record[6].to_string();
        let nome_album = record[7].to_string();
        let codigo_album = record[8].to_string();
        let ano_lancamento = record[9].to_string();
        if tipo == "M"
        {    
                let mut musica = Musica{
                    codigo: codigo_midia.clone(),
                    nome: nome.clone(),
                    duracao: duracao,
                    ano_lancamento: ano_lancamento.clone(),
                    genero: Vec::new(),
                    album_id: codigo_album.clone(),
                };

                for genero in generos
                {
                    musica.genero.push(genero.to_string());
                }
                plataforma.hash_musica.insert(record[0].to_string(), musica);


            if plataforma.hash_albuns.contains_key(&codigo_album)
            {
                let mut album_x = (plataforma.hash_albuns.get_mut(&codigo_album)).unwrap();
                album_x.qtd_musicas = album_x.qtd_musicas + 1;
                album_x.duracao = album_x.duracao + duracao;
            }
            else
            {
                let album = Album{
                    nome: nome_album.clone(),
                    codigo: codigo_album.clone(),
                    duracao: duracao,
                    ano_lancamento: ano_lancamento.clone(),
                    qtd_musicas: 1,
                    artista_id: artista_id.clone(),
                };
                plataforma.hash_albuns.insert(codigo_album.clone(), album);
            }
            if artista_id.contains(",")
            {
                let ids: Vec<&str> = artista_id.split(", ").collect();
                for id in ids
                {
                    let artista = (plataforma.hash_artista.get_mut(&id.to_string())).unwrap();
                    artista.lista_id_albuns.push(codigo_album.clone());
                    artista.lista_musicas.push(codigo_midia.clone());
                }
            }            
        }

        if tipo == "P"
        {
            let podcast = Podcast{
                    codigo: codigo_midia.clone(),
                    nome: nome.clone(),
                    duracao: duracao,
                    ano_lancamento: ano_lancamento.clone(),
                    genero: Vec::new(),
                    temporada: temporada.clone(),

            };
            plataforma.hash_podcast.insert(codigo_midia.clone(), podcast);

            if artista_id.contains(",")
            {
                let ids: Vec<&str> = artista_id.split(", ").collect();
                for id in ids
                {
                    let podcaster = (plataforma.hash_podcaster.get_mut(&id.to_string())).unwrap();
                    podcaster.lista_podcasts.push(codigo_midia.clone());
                }
            }
        }
    }
}