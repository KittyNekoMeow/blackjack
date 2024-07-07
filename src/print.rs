use crate::def::Card;

pub fn print_card(card: Card) {
    // TODO! make this not as redundant
    match card.suit {
        1 => { match card.num {
            1 => println!("Ace of hearts."),
            2 => println!("2 of hearts."),
            3 => println!("3 of hearts."),
            4 => println!("4 of hearts."),
            5 => println!("5 of hearts."),
            6 => println!("6 of hearts."),
            7 => println!("7 of hearts."),
            8 => println!("8 of hearts."),
            9 => println!("9 of hearts."),
            10 => println!("10 of hearts."),
            11 => println!("Jack of hearts."),
            12 => println!("Queen of hearts."),
            13 => println!("King of hearts."),
            _ => ()
        } },
        2 => { match card.num {
            1 => println!("Ace of diamonds."),
            2 => println!("2 of diamonds."),
            3 => println!("3 of diamonds."),
            4 => println!("4 of diamonds."),
            5 => println!("5 of diamonds."),
            6 => println!("6 of diamonds."),
            7 => println!("7 of diamonds."),
            8 => println!("8 of diamonds."),
            9 => println!("9 of diamonds."),
            10 => println!("10 of diamonds."),
            11 => println!("Jack of diamonds."),
            12 => println!("Queen of diamonds."),
            13 => println!("King of diamonds."),
            _ => ()
        } },
        3 => { match card.num {
            1 => println!("Ace of spades."),
            2 => println!("2 of spades."),
            3 => println!("3 of spades."),
            4 => println!("4 of spades."),
            5 => println!("5 of spades."),
            6 => println!("6 of spades."),
            7 => println!("7 of spades."),
            8 => println!("8 of spades."),
            9 => println!("9 of spades."),
            10 => println!("10 of spades."),
            11 => println!("Jack of spades."),
            12 => println!("Queen of spades."),
            13 => println!("King of spades."),
            _ => ()
        } },
        4 => { match card.num {
            1 => println!("Ace of clubs."),
            2 => println!("2 of clubs."),
            3 => println!("3 of clubs."),
            4 => println!("4 of clubs."),
            5 => println!("5 of clubs."),
            6 => println!("6 of clubs."),
            7 => println!("7 of clubs."),
            8 => println!("8 of clubs."),
            9 => println!("9 of clubs."),
            10 => println!("10 of clubs."),
            11 => println!("Jack of clubs."),
            12 => println!("Queen of clubs."),
            13 => println!("King of clubs."),
            _ => ()
        } },
        _ => ()
    }
}

pub fn print_cards(cards: &Vec<Card>) {
    for card in cards {
        print_card(*card);
    }
}