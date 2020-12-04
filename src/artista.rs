use crate::midias::Musica;
use crate::album::Album;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


pub struct Artista
{
    pub user_id: String,
    pub lista_id_albuns: Vec<String>,
    pub lista_musicas: Vec<String>,
    pub artist_name: String,
}

