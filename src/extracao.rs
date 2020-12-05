extern crate csv;
use csv::ReaderBuilder;
use std::env;
use crate::file;
use crate::midias;
use crate::plataformaDigital;
use crate::assinante::Assinante;
use crate::artista::Artista;
use crate::podcaster::Podcaster;
use crate::midias::{Musica, Podcast};
use crate::album::Album;
use lazy_static;
lazy_static!
{
    static ref FILE_NAME: file::File = file::set_file_names(env::args().collect());

}




pub fn adiciona_generos(plataforma: &mut plataformaDigital::PlataformaDigital)
{
    
    let path = &FILE_NAME.genero;
    let mut my_reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path).expect("Problema no csv");

    for result in my_reader.records()
    {
        let record = result.expect("quero morrer.");
        let genero = midias::Genero::new(record[1].to_string(), record[0].to_string());
        plataforma.hash_genero.insert(record[0].to_string(), genero);
    }
}


pub fn adiciona_usuarios(plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let path = &FILE_NAME.usuarios;
    let mut my_reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path).expect("Problema no csv");

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


fn add_or_update_album(record: csv::StringRecord, plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let artista_id: Vec<&str> = record[3].split(", ").collect();
    let duracao: f32 = record[4].replace(',', ".").parse().unwrap();
    let nome_album = record[7].to_string();
    let codigo_album = record[8].to_string();
    let ano_lancamento = record[9].to_string();

    if plataforma.hash_albuns.contains_key(&codigo_album)
    {
        let mut album_x = (plataforma.hash_albuns.get_mut(&codigo_album)).unwrap();
        album_x.qtd_musicas = album_x.qtd_musicas + 1;
        album_x.duracao = album_x.duracao + duracao;
    }
    else
    {
        let mut album = Album{
            nome: nome_album.clone(),
            codigo: codigo_album.clone(),
            duracao: duracao,
            ano_lancamento: ano_lancamento.clone(),
            qtd_musicas: 1,
            artista_id_vec: Vec::new(),
        };
        for id in artista_id
        {
            album.artista_id_vec.push(id.to_string());
        }
        plataforma.hash_albuns.insert(codigo_album.clone(), album);
    }
}


fn add_midia(record: csv::StringRecord, plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let codigo_midia = record[0].to_string();
    let nome = record[1].to_string();
    let duracao: f32 = record[4].replace(',', ".").parse().unwrap();
    let generos = record[5].split(", ");
    let codigo_album = record[8].to_string();
    let ano_lancamento = record[9].to_string();    
    
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
        plataforma.hash_musica.insert(codigo_midia, musica);
}


fn add_podcast(record: csv::StringRecord, plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let codigo_midia = record[0].to_string();
    let nome = record[1].to_string();
    let duracao: f32 = record[4].replace(',', ".").parse().unwrap();
    let generos = record[5].split(", ");
    let temporada = record[6].to_string();
    let ano_lancamento = record[9].to_string();
    
    let mut podcast = Podcast{
        codigo: codigo_midia.clone(),
        nome: nome.clone(),
        duracao: duracao,
        ano_lancamento: ano_lancamento.clone(),
        genero: Vec::new(),
        temporada: temporada.clone(),
    };
    
    for genero in generos
    {
        podcast.genero.push(genero.to_string());
    }
    
    plataforma.hash_podcast.insert(codigo_midia.clone(), podcast);
}

fn update_musico(record: csv::StringRecord, plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let codigo_midia = record[0].to_string();
    let codigo_album = record[8].to_string();
    let artista_id: Vec<&str> = record[3].split(", ").collect();
    for id in artista_id
    {
        let artista = (plataforma.hash_artista.get_mut(&id.to_string())).unwrap();
        artista.lista_id_albuns.push(codigo_album.clone());
        artista.lista_musicas.push(codigo_midia.clone());
    }
}


fn update_podcaster(record: csv::StringRecord, plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let codigo_midia = record[0].to_string();
    let artista_id: Vec<&str> = record[3].split(", ").collect();

    for id in artista_id
    {
        let podcaster = (plataforma.hash_podcaster.get_mut(&id.to_string())).unwrap();
        podcaster.lista_podcasts.push(codigo_midia.clone());
    }
}


pub fn adiciona_midias(plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let path = &FILE_NAME.midias;
    let mut my_reader_semicolon = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path).expect("Problema no csv");

    for result in my_reader_semicolon.records()
    {
        let record = result.expect("Trouble in paradise");
        let tipo = record[2].to_string();
        if tipo == "M"
        {    
            add_midia(record.clone(), plataforma);
            add_or_update_album(record.clone(), plataforma);
            update_musico(record.clone(), plataforma);
        }

        if tipo == "P"
        {
            add_podcast(record.clone(), plataforma);
            update_podcaster(record.clone(), plataforma);
        }
    }
}


pub fn adiciona_favoritos(plataforma: &mut plataformaDigital::PlataformaDigital)
{
    let path = &FILE_NAME.favoritos;
    let mut my_reader_semicolon = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path).expect("Problema no csv");

    for result in my_reader_semicolon.records()
    {
        let record = result.expect("Blablacar");
        let user_id = record[0].to_string();
        let midias: Vec<&str> = record[1].split(", ").collect();
        println!("O ID BUSCADO É [{}]", user_id);
        let user = plataforma.hash_assinante.get_mut(&user_id).unwrap();
        for midia in midias
        {
            user.lista_favoritos.push(midia.to_string());
        }

    }

    
}