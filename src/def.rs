/// Defines the cards of the cards.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub num: i8,
    pub suit: i8
}
/// Defines the deck.
#[derive(Debug, Clone, Copy)]
pub struct Deck {
    pub cards: [Card; 52] 
}

impl Deck {
   pub fn new() -> Self {
        Deck {
            // This is a placeholder until I can make randomized decks.
            cards: CARDS
        }
    }
}
/// Defines the shoe.
#[derive(Debug, Clone, Copy)]
pub struct Shoe4 {
    pub size: i8,
    pub decks: [Deck; 4]
}
/// Creates an array that holds every type of card.
const CARDS: [Card; 52] = [
    Card {
        num: 1,
        suit: 1
    },
    Card {
        num: 2,
        suit: 1
    },
    Card {
        num: 3,
        suit: 1
    },
    Card {
        num: 4,
        suit: 1
    },
    Card {
        num: 5,
        suit: 1
    },
    Card {
        num: 6,
        suit: 1
    },
    Card {
        num: 7,
        suit: 1
    },
    Card {
        num: 8,
        suit: 1
    },
    Card {
        num: 9,
        suit: 1
    },
    Card {
        num: 10,
        suit: 1
    },
    Card {
        num: 11,
        suit: 1
    },
    Card {
        num: 12,
        suit: 1
    },
    Card {
        num: 13,
        suit: 1
    },
    Card {
        num: 1,
        suit: 2
    },
    Card {
        num: 2,
        suit: 2
    },
    Card {
        num: 3,
        suit: 2
    },
    Card {
        num: 4,
        suit: 2
    },
    Card {
        num: 5,
        suit: 2
    },
    Card {
        num: 6,
        suit: 2
    },
    Card {
        num: 7,
        suit: 2
    },
    Card {
        num: 8,
        suit: 2
    },
    Card {
        num: 9,
        suit: 2
    },
    Card {
        num: 10,
        suit: 2
    },
    Card {
        num: 11,
        suit: 2
    },
    Card {
        num: 12,
        suit: 2
    },
    Card {
        num: 13,
        suit: 2
    },
    Card {
        num: 1,
        suit: 3
    },
    Card {
        num: 2,
        suit: 3
    },
    Card {
        num: 3,
        suit: 3
    },
    Card {
        num: 4,
        suit: 3
    },
    Card {
        num: 5,
        suit: 3
    },
    Card {
        num: 6,
        suit: 3
    },
    Card {
        num: 7,
        suit: 3
    },
    Card {
        num: 8,
        suit: 3
    },
    Card {
        num: 9,
        suit: 3
    },
    Card {
        num: 10,
        suit: 3
    },
    Card {
        num: 11,
        suit: 3
    },
    Card {
        num: 12,
        suit: 3
    },
    Card {
        num: 13,
        suit: 3
    },
    Card {
        num: 1,
        suit: 4
    },
    Card {
        num: 2,
        suit: 4
    },
    Card {
        num: 3,
        suit: 4
    },
    Card {
        num: 4,
        suit: 4
    },
    Card {
        num: 5,
        suit: 4
    },
    Card {
        num: 6,
        suit: 4
    },
    Card {
        num: 7,
        suit: 4
    },
    Card {
        num: 8,
        suit: 4
    },
    Card {
        num: 9,
        suit: 4
    },
    Card {
        num: 10,
        suit: 4
    },
    Card {
        num: 11,
        suit: 4
    },
    Card {
        num: 12,
        suit: 4
    },
    Card {
        num: 13,
        suit: 4
    },  
];