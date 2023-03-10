use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World from Github Pages with pre commit hook v2" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
