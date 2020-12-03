use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


pub struct Album
{
    pub nome: String,
    pub codigo: String,
    pub duracao: f32,
    pub ano_lancamento: String,
    pub qtd_musicas: u64,
    pub artista_id: String,
}

// impl Album
// {
//     pub fn new(nome: String, codigo: u16, duracao: f32, ano_lancamento: u16, qtd_musicas: u64, artista_id: String) -> Album
//     {
//         Album
//         {
//             nome: nome,
//             codigo: codigo,
//             duracao: duracao,
//             ano_lancamento: ano_lancamento,
//             qtd_musicas: qtd_musicas,
//             artista_id: artista_id,
//         }
//     }
//     pub fn get_nome(&self) -> &String { &self.nome }
//     pub fn get_codigo(&self) -> &u16 { &self.codigo }
//     pub fn get_duracao(&self) -> &f32 { &self.duracao }
//     pub fn get_ano_lancamento(&self) -> &u16 { &self.ano_lancamento }
//     pub fn get_qtd_musicas(&self) -> &u64 { &self.qtd_musicas }
//     pub fn get_artista_id(&self) -> &String { &self.artista_id }
// }

// impl crate::usuario::ImprimeNoArquivo for Album
// {
//     fn imprime_no_arquivo(&self, path: String) -> ()
//     {
//         let mut ofile = File::create(path).expect("unable to create file");
//         let output = "Nome: ".to_owned() + &self.get_nome() + "\n";
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Codigo: ".to_owned() + &self.get_codigo().to_string() + "\n";
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Duracao: ".to_owned() + &self.get_duracao().to_string() + "\n";
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Ano Lancamento: ".to_owned() + &self.get_ano_lancamento().to_string() + "\n";
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Quantidade de musicas: ".to_owned() + &self.get_qtd_musicas().to_string() + "\n";
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//     }
// }