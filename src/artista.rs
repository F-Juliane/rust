use crate::midias::Musica;
use crate::album::Album;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


pub struct Artista
{
    pub user_id: String,
    pub lista_id_albuns: Vec<String>,
    pub lista_musicas: Vec<String>,
    pub artist_name: String,
}

// impl Artista
// {
//     pub fn new(user_id: String, lista_id_albuns: Vec<String>) -> Artista
//     {
//         Artista
//         {
//             user_id: user_id,
//             lista_id_albuns: lista_id_albuns,
//             artist_name: "Juliane".to_string()
//         }
//     }
//     pub fn get_user_id(&self) -> &String { &self.user_id }
//     pub fn get_lista_id_albuns(&self) -> &Vec<String> { &self.lista_id_albuns }

    // pub fn imprime_no_arquivo(&self, path: String, hash_musica: HashMap<String, Musica>,) -> ()
    // {
    //     let mut ofile = File::create(path).expect("unable to create file");
    //     for (nome, musica) in hash_musica.iter()
    //     {
    //         for album_id in self.get_lista_id_albuns()
    //         {
    //             if musica.get_album_id() == album_id {
    //                 let output = "Nome: ".to_owned() + musica.get_nome() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //                 let output = "Codigo: ".to_owned() + &musica.get_codigo().to_string() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //                 let output = "Duracao: ".to_owned() + &musica.get_duracao().to_string() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //                 let output = "Ano Lancamento: ".to_owned() + &musica.get_ano_lancamento().to_string() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //                 let output = "Genero: ".to_owned() + musica.get_genero().get_nome() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //                 let output = "Sigla: ".to_owned() + musica.get_genero().get_sigla() + "\n";
    //                 ofile.write_all(output.as_bytes()).expect("unable to write");
    //             }
    //         }
    //     }     
    // }
//}