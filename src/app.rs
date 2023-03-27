use yew::prelude::*;

use crate::card::*;
// use crate::game::*;
use crate::deck::*;
use crate::player::*;


// use gloo_dialogs::alert;
// use wasm_bindgen::JsValue;
// use stylist::yew::*;
const NUMBER_OF_INITIAL_CARDS: i32 = 2;
pub enum Msg {
    Hit,
    Stand,
    Restart,
}

pub struct App {
    deck: DeckProps,
    player: PlayerHand,
    dealer: PlayerHand,
    standed: bool,
}
impl App {

    fn new() -> Self {
        let deck = DeckProps::create_cards_vector();
        let deck = DeckProps {
            cards: deck,
        };
        let player = PlayerHand { held_cards: Vec::new() };
        let dealer = PlayerHand { held_cards: Vec::new() };

        let mut output = Self {
            deck,
            player,
            dealer,
            standed: false,
        };
        output.deal_cards();

        output
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

    fn draw_card_to_hand(deck:&mut DeckProps, player_hand:&mut PlayerHand) {
        let drawn_card = deck.draw_card();
        // log!("Card drawn: \t{}", drawn_card.name.as_str());
        player_hand.add_card_to_hand(drawn_card);

    }

    fn initialize_player_hand(&mut self, number_of_cards: i32) -> Vec<Card> {
        let mut output: Vec<Card> = Vec::new();
        for _ in 0..number_of_cards {

        let new_card = self.deck.draw_card();
        output.push(new_card);
        }
        output
    }
    fn player_hit(&mut self) -> bool { 
        // println!("player hit");
        Self::draw_card_to_hand(&mut self.deck, &mut self.player); 
        self.player.is_busted() 
    }

    fn dealer_plays(&mut self) -> bool {
        // dealer draws until he has a score of 17 or more
        loop {
            // println!("Dealer hand: \t{}", self.dealer.get_hand_total());
            if self.dealer.get_hand_total() >= 17 {
                return true;
            }
            Self::draw_card_to_hand(&mut self.deck, &mut self.dealer);
        }
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hit => {
                let _ = self.player_hit();
                true
            }
            Msg::Stand => {
                self.standed = true;
                let _ = self.dealer_plays();
                true
            }
            Msg::Restart => {
                self.standed = false;
                let new = Self::new();
                self.deck = new.deck;
                self.player = new.player;
                self.dealer = new.dealer;

                true
            }

        }

            
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
         html! {
            <>
                <div class="w3-container">
                    <h2> {"Dealer Hand"} </h2>
                </div>

                <Player held_cards={self.dealer.held_cards.clone()}/>

                <div class="w3-container">
                    <h2> {"Player Hand"} </h2>
                </div>

                <Player held_cards={self.player.held_cards.clone()}/>

                <div class="buttons">

                if !&self.player.is_busted() {
                    if self.dealer.is_busted() {
                        <h1> { " You Win " } </h1>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Restart)}>
                            { "Restart" }
                        </button>
                    }
                    else {
                        if !&self.standed {
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Hit)}>
                            { "Hit" }
                        </button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Stand)}>
                            { "Stand" }
                        </button>
                        }
                    }
                }
                else {
                    <h1> { " Dealer Wins " } </h1>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Restart)}>
                        { "Restart" }
                    </button>
                }

                </div>
            </>
        }

    }

}

pub fn render() {
    
    yew::Renderer::<App>::new().render();
}
