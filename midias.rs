trait ImprimeQtdProdutos
{
   fn imprime_qtd_produtos(&self) -> ();
}

trait FormataDuracao
{
    fn formata_duracao(&self) -> String;
}

trait ImprimeInfoMidia
{
    fn imprime_info_midia(&self) -> ();
}

trait ImprimeNoArquivo
{
    fn imprime_no_arquivo(&self, output: String) -> ();
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

pub struct Podcast
{
    pub temporada: String,
    pub nome: String,
    pub codigo: String,
    pub duracao: f32,
    pub ano_lancamento: String,
    pub genero: Vec<String>,
}

