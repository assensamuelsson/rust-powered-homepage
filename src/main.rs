use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World from Github Pages" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
