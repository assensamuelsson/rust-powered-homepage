use yew::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <img class="logo" src="/rust-powered-homepage/assen.png" />
    }
}
