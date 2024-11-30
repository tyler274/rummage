mod card;
mod cards;
mod mana;
mod player;

use bevy::prelude::*;
use mana::Mana;

fn hello_world() {
    println!("hello world!");
    println!("Mana default color is: {:?}", Mana::default());
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
