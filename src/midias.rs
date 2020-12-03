use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};
trait ImprimeQtdProdutos
{
   fn imprime_qtd_produtos(&self) -> ();
}

trait FormataDuracao
{
    fn formata_duracao(&self) -> String;
}

trait imprimeInfoMidia
{
    fn imprime_info_midia(&self) -> ();
}

trait ImprimeNoArquivo
{
    fn imprime_no_arquivo(&self, output: String) -> ();
}


pub struct Midia
{
    nome: String,
    codigo: u16,
    duracao: f32,
    ano_lancamento: u16,
    genero_id: String,
}

pub struct Genero {
    pub nome: String,
    pub sigla: String,
}
impl Genero {
    pub fn new(nome: String, sigla: String) -> Genero {
        Genero {
            nome: nome,
            sigla: sigla,
        }
    }
}

pub struct Musica { 
    pub nome: String,
    pub codigo: String,
    pub duracao: f32,
    pub ano_lancamento: String,
    pub genero: Vec<String>,
    pub album_id: String,
}

// impl Musica {
//     pub fn new(nome: String, codigo: i64, duracao: f64, ano_lancamento: i64, genero: Genero, album_id: String) -> Musica {
//         Musica {
//             nome: nome,
//             codigo: codigo,
//             duracao: duracao,
//             ano_lancamento: ano_lancamento,
//             genero: genero,
//             album_id: album_id,
//         }
//     }
//     pub fn get_nome(&self) -> &String { &self.nome }
//     pub fn get_codigo(&self) -> &i64 { &self.codigo }
//     pub fn get_duracao(&self) -> &f64 { &self.duracao }
//     pub fn get_ano_lancamento(&self) -> &i64 { &self.ano_lancamento }
//     pub fn get_genero(&self) -> &Genero { &self.genero }
//     pub fn get_album_id(&self) -> &String { &self.album_id }
//     pub fn imprime_info_produto(&self) -> () {
//         println!("Nome: {}", self.nome);
//         println!("Nome do genero: {}", self.genero.nome);
//         println!("Sigla do genero: {}", self.genero.sigla);
//         println!("Duracao: {}", self.duracao);
//         println!("Ano de lancamento: {}", self.ano_lancamento);
//     }
//     pub fn imprime_no_arquivo(&self, path: String) -> () {
//         let mut ofile = File::create(path).expect("unable to create file");
//         let output = "Nome: ".to_owned() + &self.get_nome();
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         // let output = "Nome do genero: ".to_owned() + &self.get_genero().get_nome();
//         // ofile.write_all(output.as_bytes()).expect("unable to write");
//         // let output = "Sigla do genero: ".to_owned() + &self.get_genero().get_sigla();
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Duracao: ".to_owned() + &self.get_duracao().to_string();
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//         let output = "Ano de lancamento: ".to_owned() + &self.get_ano_lancamento().to_string();
//         ofile.write_all(output.as_bytes()).expect("unable to write");
//     }
// }

pub struct Podcast
{
    pub temporada: String,
    pub nome: String,
    pub codigo: String,
    pub duracao: f32,
    pub ano_lancamento: String,
    pub genero: Vec<String>,
}

