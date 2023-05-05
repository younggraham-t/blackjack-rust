# Blackjack Written in Rust
A simple blackjack web app written in Rust using the Yew front-end framework for my CSC3320 class.  
Details for running at the bottom of this document.
 
--- 
## Deliverable Requirements
### Demo:
[https://youtu.be/xQD-XyN2MN8](https://youtu.be/xQD-XyN2MN8)

### Language:
- This project was written in Rust. I did not know Rust prior to creating this project.
- Rust is a compiled, high-level, multi-paradigm language however it is primarily used for functional style programming. It is procedural.
### Program:
The program that I wrote is a blackjack web app. The project follows the MVVM design pattern however the code is not organized into files this way. The core of the app is a struct called App
```rust
pub struct App {
    deck: DeckProps,
    player: PlayerHand,
    dealer: PlayerHand,
    standed: bool,
    dealer_standed: bool,
    // dealer_face_down_card: &'a Card,
}
```
which implements a Trait (similar to an interface in Java) from Yew called Component
```rust
impl Component for App {
    ...
}
```
that gives it a view and update function 
```rust
fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
	...
}
fn view(&self, ctx: &Context<Self>) -> Html {
// Yew adds a macro that can be used to directly write html and yew components
	html! {  
		<div> ... </div> //this will compile directly to html code

		// The following doesn't work in html but it does work here because it is 
		// part of Yew

		// Player is a struct I wrote
	    	<Player held_cards={self.player.held_cards.clone()}/> 

		...
	}
}
```
Yew also adds a Trait that allows for a struct to be used in the html macro and then compiled into javascript code.
```rust
// styled_component is not actually from the Yew base but works similarly to 
// function_component but adds the ability to format it with css

#[styled_component(Player)] //this is a macro that automatically implements component for a struct called Player
pub fn player(PlayerHand { held_cards }: &PlayerHand) -> Html {
// in Rust every line must end in ; but if you dont end a line with ; it will 
// treat it as a return
    held_cards.iter() //iter is similar to .stream in Java but much more powerful
        .map(|card| { // this is the syntax for a lambda function (|_| {})
            html! {
                <CardDetails card={card.clone()} /> // another struct I wrote
            }
    }).collect() // no ; which means it returns the result of the iter.
}
```
Those are just a few of the most important things to know in order to understand my code.

---
## Running
### To Run:
1. Ensure you have Rust installed if it's not:  
for macOS, Linux run:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
in the terminal or follow the instructions here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) if you are on Windows.  

2. Install Trunk using:
```
cargo install --locked trunk
```
3. Clone the repository using:
```
git clone https://github.com/younggraham-t/blackjack-rust.git
```
4. Then run:
```
trunk serve --open
```
from the root directory of the project. This will build the project, start a local server on 127.0.0.1, and open it in your browser.





