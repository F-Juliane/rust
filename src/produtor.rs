use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};

pub struct Produtor
{
    user_id: String,
}

// trait ImprimeProdutosDesenvolvidos
// {
//     fn imprime_produtos_desenvolvidos(&self, String) -> ();
// }

impl Produtor
{
    pub fn imprime_produtos_desenvolvidos(&self, output: String) -> ()
    {
        let mut f = File::create("../files/test_prod.txt")
            .expect("could not open file");
        f.write_all(output.as_bytes())
            .expect("Deu n, mano");
        
    }
}

fn main()
{
    let produtor_san = Produtor{user_id: "NGSUHATE".to_string()};
    produtor_san.imprime_produtos_desenvolvidos("AAAAAAAAAAAAAAAAAAAAA".to_string());
}