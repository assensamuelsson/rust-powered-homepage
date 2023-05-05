use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let parapgraphs = vec![
        "My name is Andreas Samuelsson and I welcome you to my homepage.",
        "I'm a software developer, singer songwriter and water colour painter. I live in Norrköping, Sweden with my wife and children.",
        "You will find that this page is rather empty at the moment, but I will try to add content as time goes by. For the time being you can check out my blog.",
    ];

    html! {
        <article>
            { parapgraphs.iter().map(|p| html! { <p>{ p }</p> }).collect::<Html>() }
        </article>
    }
}
