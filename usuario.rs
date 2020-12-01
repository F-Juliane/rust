use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


pub struct Usuario
{
    pub nome: String,
    pub codigo: u16,
}

pub trait ImprimeNoArquivo
{
    fn imprime_no_arquivo(&self, String) -> ();
}

impl ImprimeNoArquivo for Usuario
{
    fn imprime_no_arquivo(&self, output: String) -> ()
    {
        //println!("teste");
        let mut f = File::create("../files/test.txt")
            .expect("could not open file");
        f.write_all(output.as_bytes())
            .expect("Deu n, mano");
        
    }
}
