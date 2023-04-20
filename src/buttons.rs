// use std::fmt;
// use yew::prelude::*;
// use stylist::yew::*;
// use crate::app::*;
//
// #[derive(Properties, PartialEq)]
// struct ButtonProps<'a> {
//     ctx: &'a Context<App>,
//     is_displayed: bool,
// }
// #[styled_component(HitButton)]
// pub fn hitButton(ButtonProps { ctx, is_displayed }: &ButtonProps) -> Html {
//     html! {
//     <button class="button" onclick={
//                             ctx.link().callback(|_| Msg::Hit)
//                         }>
//                             { "Hit" }
//                         </button>
//     }
// }
//
