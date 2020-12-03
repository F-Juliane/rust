pub struct Assinante
{
    pub user_id: String,
    pub lista_favoritos: Vec<String>,
    pub user_name: String,
}


fn main()
{
    // let usr = usuario::Usuario {nome: "Daniel".to_string(), codigo: 42};
    let ass = Assinante{user_id: "L".to_string(), lista_favoritos: Vec::new(), user_name: "J".to_string()};
    //ass.user.imprime_no_arquivo("Kelbin".to_string());
}   