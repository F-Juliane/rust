use crate::plataforma_digital;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_sorted_prod_vec(plataforma: &plataforma_digital::PlataformaDigital) 
    -> Vec<(String, String, &Vec<String>, String)>
{
    let mut prod_vec: Vec<(String, String, &Vec<String>, String)> = Vec::new();
    for (id, podcaster) in plataforma.hash_podcaster.iter()
    {
        prod_vec.push((id.to_string(), podcaster.podcaster_name.clone(), &podcaster.lista_podcasts, "P".to_string()));
    }

    for (id, artista) in plataforma.hash_artista.iter()
    {
        prod_vec.push((id.to_string(), artista.artist_name.clone(), &artista.lista_musicas, "A".to_string()));
    }
    prod_vec.sort_by(|a,b| a.1.cmp(&b.1));
    return prod_vec;
}

fn get_sorted_users_vec(plataforma: &plataforma_digital::PlataformaDigital) 
    -> Vec<(String, String, String)>
{
    let mut users_vec: Vec<(String, String, String)> = Vec::new();
    for (id, assinante) in plataforma.hash_assinante.iter()
    {
        users_vec.push((id.to_string(), assinante.user_name.clone(), "U".to_string()));
    }

    for (id, podcaster) in plataforma.hash_podcaster.iter()
    {
        users_vec.push((id.to_string(), podcaster.podcaster_name.clone(), "P".to_string()));
    }

    for (id, artista) in plataforma.hash_artista.iter()
    {
        users_vec.push((id.to_string(), artista.artist_name.clone(), "A".to_string()));
    }
    users_vec.sort_by(|a,b| a.0.parse::<i32>().unwrap().cmp(&b.0.parse::<i32>().unwrap()));
    return users_vec;
}

fn get_sorted_all_midias_vec(plataforma: &plataforma_digital::PlataformaDigital) 
    -> Vec<(String,String,String,String,Vec<String>,f32,Vec<String>,String,String,String,String)>
{
    let mut midias_vec: Vec<(String,String,String,String,Vec<String>,f32,Vec<String>,String,String,String,String)> = Vec::new();
    for (id, musica) in plataforma.hash_musica.iter()
    {
        midias_vec.push((id.to_string(), musica.codigo.clone(), musica.nome.clone(),"M".to_string(),get_producters_vec(plataforma,id,"M"),musica.duracao, musica.genero.clone(), "".to_string(),get_album_name(plataforma,&musica.album_id),musica.album_id.clone(),musica.ano_lancamento.clone()));
    }
    for (id, podcast) in plataforma.hash_podcast.iter()
    {
        midias_vec.push((id.to_string(), podcast.codigo.clone(), podcast.nome.clone(),"P".to_string(),get_producters_vec(plataforma,id,"P"),podcast.duracao, podcast.genero.clone(), podcast.temporada.clone(),"".to_string(),"".to_string(),podcast.ano_lancamento.clone()));
    }
    midias_vec.sort_by(|a,b| a.0.parse::<i32>().unwrap().cmp(&b.0.parse::<i32>().unwrap()));
    return midias_vec;
}

fn get_sorted_midias_vec(plataforma: &plataforma_digital::PlataformaDigital, midias: &Vec<String>, tipo: &str)
    -> Vec<String>
{
    let mut midias_nm_vec = Vec::new();
    if tipo == "A"
    {
        for musica in midias.iter()
        {
            let nome_musica = plataforma.hash_musica.get(musica).unwrap().nome.clone();
            midias_nm_vec.push(nome_musica.clone())
        }
        midias_nm_vec.sort_by(|a,b| a.cmp(&b));

    }
    if tipo == "P"
    {
        for podcast in midias.iter()
        {
            let nome_podcast = plataforma.hash_podcast.get(podcast).unwrap().nome.clone();
            midias_nm_vec.push(nome_podcast.clone())
        }
        midias_nm_vec.sort_by(|a,b| a.cmp(&b));
    }
    return midias_nm_vec;
}


pub fn midias_por_produtor(plataforma: &plataforma_digital::PlataformaDigital)
{
    let path = Path::new("relatorios/midias_por_produtor.csv");
    let mut file = File::create(&path).expect("Não sei escrever");
    let prod_vec = get_sorted_prod_vec(plataforma);

    file.write_all("Nome;Midias\n".as_bytes()).expect("Sou analfabeto");
    for prod in prod_vec.iter()
    {   
        let midias_nm_vec = get_sorted_midias_vec(plataforma, &prod.2, &prod.3);
        let mut output = prod.1.clone() + ";";
        
        if ! midias_nm_vec.is_empty()
        {
            for musica in midias_nm_vec.iter()
            {
                output = output + musica + "; ";
            }
            output.pop();
            output.pop();
        }

        output = output + "\n";
        file.write_all(output.as_bytes()).expect("Marrapais");
    }
}


pub fn faz_backup_usuarios(plataforma: &plataforma_digital::PlataformaDigital)
{   
    let path = Path::new("relatorios/backup_usuarios.csv");
    let mut file = File::create(&path).expect("Eu queria ser bonito");
    let users_vec = get_sorted_users_vec(plataforma);

    file.write_all("Código;Tipo;Nome\n".as_bytes()).expect("Sou analfabeto");
    for user in users_vec.iter()
    {
        let output = user.0.clone() + ";" + &user.2.clone() + ";" + &user.1.clone() + "\n";
        file.write_all((output).as_bytes()).expect("Estou triste");
    }
}

fn get_producters_vec(plataforma: &plataforma_digital::PlataformaDigital, 
    midia_id: &str, tipo: &str) -> Vec<String>
{
    let mut prod_vec = Vec::new();
    if tipo == "M"
    
    {
        for (_, artista) in plataforma.hash_artista.iter()
        {
            for midia in &artista.lista_musicas
            {
                if midia == midia_id
                {
                    prod_vec.push(artista.user_id.clone());
                    break;
                }
            }
        }
    }

    if tipo == "P"
    {
        for (_, podcaster) in plataforma.hash_podcaster.iter()
        {
            for midia in &podcaster.lista_podcasts
            {
                if midia == midia_id
                {
                    prod_vec.push(podcaster.user_id.clone());
                    break;
                }
            }
        }
    }
    
    return prod_vec;
}


fn get_album_name(plataforma: &plataforma_digital::PlataformaDigital, album_id: &str) -> String
{
    let album = plataforma.hash_albuns.get(album_id).unwrap();
    return album.nome.clone()
}


pub fn faz_bakcup_midias(plataforma: &plataforma_digital::PlataformaDigital)
{
    let path = Path::new("relatorios/backup_midias.csv");
    let mut file = File::create(&path).expect("Eu queria ser bonito");
    let midias_vec = get_sorted_all_midias_vec(plataforma);
    file.write_all("Código;Nome;Tipo;Produtores;Duração;Gênero;Temporada;Álbum;Código do Álbum;Publicação".as_bytes()).expect("argh");
    file.write_all("\n".as_bytes()).expect("Gorillaz");
    let mut output = String::new();

    for midia in midias_vec.iter()
    {
        let prod_vec = get_producters_vec(plataforma, &midia.0, &midia.3);
        output = output + &midia.1 + ";" + &midia.2 + ";" + &midia.3 + ";";
        for prod in prod_vec.iter()
        {
            output = output + prod + ", "
        }
        output.pop(); // Remove o whitespace extra
        output.pop(); // Remove vírgula extra
        output = output + ";" + &midia.5.to_string() + ";";
        for genero in midia.6.iter()
        {
            output = output + &genero + ", "
        }
        output.pop(); // Remove o whitespace extra
        output.pop(); // Remove a vírgula extra

        if midia.3.clone() == "M"
        {
            output = output + ";" + ";" + &get_album_name(plataforma, &midia.9) + ";" + &midia.9 + 
            ";" + &midia.10;
            output = output + "\n";
        }
        if midia.3.clone() == "P"
        {
            output = output + ";" + &midia.7 + ";" + ";" + ";" + &midia.10;
            output = output + "\n";
        }
    }

    file.write_all(output.as_bytes()).expect("It is the end");
}