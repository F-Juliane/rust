use std::fs::File;
use std::io::Write;

pub struct Produtor
{
    user_id: String,
}

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