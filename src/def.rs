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
            cards: shuffle(CARDS)
        }
    }
}
/// Defines the shoe.
#[derive(Debug, Clone)]
pub struct Shoe {
    pub size: i8,
    pub all_cards: Vec<Card>
}

impl Shoe {
    pub fn new(num_decks: i8) -> Self {
        let mut i = num_decks;
        let mut cards: Vec<Card> = Vec::new();
        while i != 0 {
            let new_deck = Deck::new();
            for card in new_deck.cards {
                cards.push(card);
            }
            i -= 1;
        }
        Shoe {
            size: num_decks,
            all_cards: cards
        }
    }
    pub fn hit(&mut self) -> Card {
        let card = self.all_cards.pop().unwrap();
        return card;
    }
}
/// Creates an array that holds every type of card.
const CARDS: [Card; 52] = [
    Card {
        // 1 are aces
        num: 1,
        // 1 are hearts
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
        // 11 are jacks
        num: 11,
        suit: 1
    },
    Card {
        // 12 are queens
        num: 12,
        suit: 1
    },
    Card {
        // 13 are kings
        num: 13,
        suit: 1
    },
    Card {
        num: 1,
        // 2 are diamonds
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
        // 3 are spades
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
        // 4 are clubs
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
/// Radnomizes the cards in a deck.
fn shuffle(mut deck: [Card; 52]) -> [Card; 52] {
    // Grabs the length of the deck as f32.
    let mut current_index = deck.len() as f32;

    while current_index != 0.0 {
        // Grabs a random float.
        let random_number: f32 = rand::random();
        // Multiplies the current index by the random float.
        // Floors the result and makes it a random index.
        let random_index = (random_number * current_index).floor();
        // Subtracts 1 to the current index.
        current_index -= 1.0;
        // Swaps the numbers at the corresponding indexes.
        (deck[current_index as usize], deck[random_index as usize]) = 
        (deck[random_index as usize], deck[current_index as usize]);
    }

    return deck;
}