// use std::fmt::DebugList;

use yew::prelude::*;
use stylist::css;
use stylist::yew::Global;
use crate::card::*;
// use crate::game::*;
use crate::deck::*;
use crate::player::*;
// use crate::buttons::*;

// const PLACE_HOLDER_CARD: &'static Card = & Card
// { id: 0, value: Value::Ace, suit: Suit::Spades, name: String::new(), is_face_up: false };

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
    dealer_standed: bool,
    game_ended: bool,
    // dealer_face_down_card: &'a Card,
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
            dealer_standed: false,
            game_ended: false,
            // dealer_face_down_card: PLACE_HOLDER_CARD,
        };
        output.deal_cards();

        output
    }

    fn deal_cards(&mut self) {
        let player_hand = self.initialize_player_hand(NUMBER_OF_INITIAL_CARDS);
        for card in player_hand {
            self.player.add_card_to_hand(card);
        }

        let dealer_hand = self.initialize_player_hand(NUMBER_OF_INITIAL_CARDS -1);

        let mut new_card: Card = self.deck.draw_card();
        // let new_card_ref: &'a Card = &new_card;
        // self.dealer_face_down_card = new_card_ref;
        new_card.is_face_up = false;
        self.dealer.add_card_to_hand(new_card);
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
        if self.player.is_busted() {
            self.dealer.face_up_all_cards();
        }
        self.player.is_busted()
    }


    fn dealer_plays(&mut self) -> bool {
        // dealer draws until he has a score of 17 or more
        loop {
            // println!("Dealer hand: \t{}", self.dealer.get_hand_total());
            if self.dealer.get_hand_total() >= 17 {
                self.dealer.face_up_all_cards();
                self.game_ended = true;
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
                if self.player.is_busted() {
                    self.game_ended = true;
                }
                true
            }
            Msg::Stand => {
                self.standed = true;
                self.dealer_standed = self.dealer_plays();
                true
            }
            Msg::Restart => {
                self.standed = false;
                self.dealer_standed = false;
                let new = Self::new();
                self.deck = new.deck;
                self.player = new.player;
                self.dealer = new.dealer;
                self.game_ended = false;

                true
            }

        }


    }

    fn view(&self, ctx: &Context<Self>) -> Html {
         html! {
            <>
                <Global css={css!(r#"display:grid; 
                grid-template-rows:auto auto auto;
                margin: 10px;
                "#)} />

                <section class={css!("margin:10px;")}>
                <h2> {"Dealer Hand"} </h2>
                <div class={css!("display:flex;margin:10px;")}>
                    <Player held_cards={self.dealer.held_cards.clone()}/>
                </div>


                
                <div class={css!("display:flex;margin:10px;")}>
                    <Player held_cards={self.player.held_cards.clone()}/>
                </div>
                <h2> {"Player Hand"} </h2>
                </section>
                

                <section class={css!("justify-content:right;")}>

                <button class="button" disabled={!self.game_ended} onclick={
                    ctx.link().callback(|_| Msg::Restart)
                }>
                    { "Restart" }
                </button>
                <button class="button" disabled={self.game_ended} onclick={
                    ctx.link().callback(|_| Msg::Hit)
                }>
                    { "Hit" }
                </button>
                <button class="button" disabled={self.standed || self.game_ended} onclick={
                    ctx.link().callback(|_| Msg::Stand)
                }>
                    { "Stand" }
                </button>
                if !&self.player.is_busted() {
                    if self.dealer.is_busted() {
                        <h1> { " You Win " } </h1>

                    }
                    else {
                        
                        if self.dealer_standed {
                            
                            if self.player.get_hand_total() > self.dealer.get_hand_total() {
                                <h1> { " You Win " } </h1>
                            }
                            else if self.player.get_hand_total() == self.dealer.get_hand_total() {
                                <h1> { " Push " } </h1>
                            }
                            else {
                                <h1> { " Dealer Wins " } </h1>
                            }
                        }
                    }
                }
                else {
                    <h1> { " Dealer Wins " } </h1>
                }

                </section>
            </>
        }

    }

}

pub fn render() {

    let app: yew::Renderer::<App> = yew::Renderer::<App>::new();
    app.render();
}
