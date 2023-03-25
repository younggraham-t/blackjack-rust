use super::player::PlayerHand;
use crate::deck::DeckProps;
use crate::Card;
use std::io::stdin;
use std::collections::HashMap;

const NUMBER_OF_INITIAL_CARDS: i32 = 2;
// const BLACKJACK: i32 = 21;

pub struct Dealer {
    deck: DeckProps,
    player: PlayerHand,
    dealer: PlayerHand,
}


impl Dealer {
    pub fn new(deck: DeckProps) -> Self {

        Self {
            deck,
            player: PlayerHand::new(),
            dealer: PlayerHand::new(),
        } 
        
    }

    fn deal_cards(&mut self) {
        let player_hand = self.initialize_player_hand(NUMBER_OF_INITIAL_CARDS);
        for card in player_hand {
            self.player.add_card_to_hand(card);
        }

        let dealer_hand = self.initialize_player_hand(NUMBER_OF_INITIAL_CARDS);
        for card in dealer_hand {
            self.dealer.add_card_to_hand(card);
        }

    }
    
    pub fn run(&mut self) {
        self.deal_cards();
        let mut user_options: HashMap<&str, fn(&mut Dealer) -> bool> = HashMap::new();
        // type Binop = fn();
        // let dp: Binop = self.dealer_plays();
        user_options.insert("stand", Dealer::dealer_plays);
        user_options.insert("hit", Dealer::player_hit);
        
        let dealer_start_total = self.dealer.get_hand_total();
        println!("Delear hand: \t{}\nDealer total: \t{}", self.dealer, dealer_start_total);
        loop {
            let player_current_total = self.player.get_hand_total();
            println!("Player hand: \t{}\nPlayer total: \t{}", self.player, player_current_total);
            let mut player_input = String::new();
            println!("Do you wish to 'hit' or 'stand'?  Type 'q' to quit.");
            stdin().read_line(&mut player_input).expect("yes");
            player_input = player_input.trim().to_lowercase();
            // println!("{}", player_input);
            if player_input.as_str() == "q" {
                return;
            }
            match user_options.get(&player_input.as_str()) {
                Some(func) => {let end = func(self);
                            if end {
                                //println!("You lose");
                                break;
                            }
                            else {
                                continue;
                            }
                        },
                None => println!("Enter either 'hit' or 'stand'."),
            }
        }
        
        if self.player.is_busted() {
            println!("Dealer wins");
            
        } else if self.dealer.is_busted() {
            println!("You win");
        } else {
            let win = self.player.get_hand_total() > self.dealer.get_hand_total();
            if win {
                println!("You win");
            } else {
                println!("Dealer wins");
            }
        }

    }

    fn draw_card_to_hand(deck:&mut DeckProps, player_hand:&mut PlayerHand) {
        let drawn_card = deck.draw_card();
        println!("Card drawn: \t{}", drawn_card.name.as_str());
        player_hand.add_card_to_hand(drawn_card);

    }

    fn player_hit(&mut self) -> bool { 
        // println!("player hit");
        Self::draw_card_to_hand(&mut self.deck, &mut self.player); 
        self.player.is_busted() 
    }

    fn dealer_plays(&mut self) -> bool {
        // dealer draws until he has a score of 17 or more
        loop {
            println!("Dealer hand: \t{}", self.dealer.get_hand_total());
            if self.dealer.get_hand_total() >= 17 {
                return true;
            }
            Self::draw_card_to_hand(&mut self.deck, &mut self.dealer);
        }
        // false
    }


    fn initialize_player_hand(&mut self, number_of_cards: i32) -> Vec<Card> {
        let mut output: Vec<Card> = Vec::new();
        for _ in 0..number_of_cards {

        let new_card = self.deck.draw_card();
        output.push(new_card);
        }
        output
    }
}


