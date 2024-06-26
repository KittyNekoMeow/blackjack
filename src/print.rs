use crate::def::Card;

pub fn print_card(card: Card) {
    match card.suit {
        1 => println!("{} of hearts", card.num),
        2 => println!("{} of diamonds", card.num),
        3 => println!("{} of spades", card.num),
        4 => println!("{} of clubs", card.num),
        _ => ()
    }
}