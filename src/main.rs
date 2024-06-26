use def::Deck;
use print::print_card;

mod def;
mod print;
fn main() {
    let deck = Deck::new();

    for card in deck.cards {
        print_card(card);
    }
}