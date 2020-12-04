use crate::plataformaDigital;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_sorted_prod_vec(plataforma: &plataformaDigital::PlataformaDigital) 
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


fn get_sorted_midias_vec(plataforma: &plataformaDigital::PlataformaDigital, midias: &Vec<String>, tipo: &str)
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


pub fn midias_por_produtor(plataforma: &plataformaDigital::PlataformaDigital)
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
                output = output + musica + ", ";
            }
            output.pop();
            output.pop();
        }

        output = output + "\n";
        file.write_all(output.as_bytes()).expect("Marrapais");
    }

}

