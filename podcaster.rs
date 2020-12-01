use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


pub struct Podcaster
{
    user_id: String,
}


// Redefinição para caso de "sobrecarga"
impl Podcaster
{
    pub fn imprime_no_arquivo(&self, output: String) -> ()
    {
        println!("teste");
    }
}


fn main()
{
    let azg = Podcaster{user_id: "EL PSY CONGROO".to_string()};
    azg.imprime_no_arquivo("Fer".to_string());
}   