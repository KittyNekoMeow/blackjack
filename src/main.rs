use def::Shoe;
use print::print_card;
use std::io;

mod def;
mod print;
fn main() {
    let mut shoe = Shoe::new(1);

    loop {
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    
    if player_input_clean == "hit".to_string() {
        print_card(shoe.hit());
    }
}
}