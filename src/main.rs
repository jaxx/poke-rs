extern crate poke_rs;
extern crate serde_json;

use std::{env, process};
use std::fs::File;
use std::io::{BufReader, Read};

use poke_rs::Pokemon;
use poke_rs::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });




    // https://pokemon.gameinfo.io
    // 1. load pokemon data file
    // 2. load moves data file

    let pokemon_json_file = match File::open("data/pokemon.json") {
        Ok(f) => f,
        Err(e) => panic!("Ei suutnud faili laadida: {}", e)
    };

    let mut contents = String::new();

    BufReader::new(pokemon_json_file).read_to_string(&mut contents);

    let pokemons: Vec<Pokemon> = serde_json::from_str(&contents).unwrap();

    println!("{:?}", pokemons);
}
