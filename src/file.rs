pub struct File
{
    pub genero: String,
    pub midias: String,
    pub usuarios: String,
    pub favoritos: String,
}


pub fn set_file_names(args: Vec<String>) -> File
{
    println!("to aqui\n");
    let mut genero = "entradas/generos.csv".to_string();
    let mut midias = "entradas/midias.csv".to_string();
    let mut usuarios = "entradas/usuarios.csv".to_string();
    let mut favoritos = "entradas/favoritos.csv".to_string();
    let directory = "entradas/".to_string();
    
    for(i, arg) in args.iter().enumerate()
    {
        match arg.as_str()
        {
            "g" => genero = directory.clone() + &args[i+1],
            "u" => usuarios = directory.clone() + &args[i+1],
            "m" => midias = directory.clone() + &args[i+1],
            "f" => favoritos = directory.clone() + &args[i+1],
            _ => {}

        }
        
    }
    return File{genero:genero, midias:midias, usuarios:usuarios, favoritos:favoritos};
}