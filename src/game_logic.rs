use crate::{def::{get_input, Card}, gamestate::Gamestate, print::print_cards};

fn get_point(card: Card) -> i8 {
    match card.num {
        1 => 1,
            // TODO! make it so that you can choose the ace number
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        10 => 10,
        11 => 10,
        12 => 10,
        13 => 10,
        _ => 0
    }
}
/// Adds all the points together.
fn count_points(cards: &Vec<Card>) -> i8 {
    let mut total = 0;

    for card in cards {
        total += get_point(*card)
    }
    return total;
}
/// Checks if your point total is over 21.
fn did_bust(cards: &Vec<Card>) -> bool {
    let points = count_points(cards);

    if points > 21 {
        return true;
    } else {
        return false;
    }
}
/// The main function that plays the game.
pub fn play_game() {
    // Create the collection of cards.
    let mut player_cards: Vec<Card> = Vec::new();
    let mut dealer_cards: Vec<Card> = Vec::new();
    // Inizilize the variables that hold if someone won.
    let mut p_win = false;
    let mut d_win = false;
    let mut gamestate = Gamestate::new();
    // Start the game with 2 cards each.
    dealer_cards.push(gamestate.shoe.hit());
    dealer_cards.push(gamestate.shoe.hit());
    player_cards.push(gamestate.shoe.hit());
    player_cards.push(gamestate.shoe.hit());
    // Print their cards.
    // TODO! make it so that the second card of the dealer is invisible
    print_cards(&dealer_cards);
    print_cards(&player_cards);
    loop {
        // If you hit blackjack you win.
        if count_points(&player_cards) == 21 {
            println!("You win.");
            break;
        }
        // Grabs player input.
        let input = get_input();
        if input == "hit".to_string() {
            // If input is hit get a new card.
            player_cards.push(gamestate.shoe.hit());
            // Print the card.
            print_cards(&player_cards);
            // Check if you hit blackjack.
            if count_points(&player_cards) == 21 {
                println!("You win.");
                p_win = true;
                break;
                // Check if you went over 21 points.
            } else if did_bust(&player_cards) {
                println!("You bust.");
                d_win = true;
                break;
            }  
            // If input is stand break from this loop.          
        } else if input == "stand".to_string() {
            break;
        }
    }
    // Calculate the players score.
    let player_score = count_points(&player_cards);
    loop {
        // If player won break from this loop.
        if p_win {
            break;
        } 
        // If you bust in the previous loop break from this loop.
        if d_win {
            break;
        }
        // If dealer busts player win.
        if did_bust(&dealer_cards) {
            println!("You win.");
            break;
        }
        // If the dealer hasen't busted and has a greater score than the player.
        // The dealer wins.
        if count_points(&dealer_cards) > player_score {
            println!("You lose.");
            break;
        }
        // The dealer grabs a new card and it gets printed.
        dealer_cards.push(gamestate.shoe.hit());
        print_cards(&dealer_cards);
    }
}