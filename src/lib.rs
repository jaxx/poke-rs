#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    id: u16,
    dex_num: String,
    name: String,
    attack: u16,
    defense: u16,
    hp: u16,
    types: Vec<Type>
}

#[derive(Deserialize, Debug)]
enum Type {
    Fire,
    Flying,
    Grass,
    Poison,
    Water
}
