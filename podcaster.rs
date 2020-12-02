pub struct Podcaster
{
    pub user_id: String,
    pub podcaster_name: String,
    pub lista_podcasts: Vec<String>,
}


// Redefinição para caso de "sobrecarga"
impl Podcaster
{
    pub fn imprime_no_arquivo(&self, output: String) -> ()
    {
        println!("{}", output);
    }
}