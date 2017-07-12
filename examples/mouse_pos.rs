extern crate enigo;
use enigo::{Enigo, MouseControllable};

fn main() {
    let mut enigo = Enigo::new();

    println!("({:?})", enigo.mouse_get_pos());
}
