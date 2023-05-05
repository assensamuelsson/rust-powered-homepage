use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/rust-powered-homepage/")]
    Home,
    #[at("/rust-powered-homepage/blog")]
    Blog,
    #[at("/rust-powered-homepage/gallery")]
    Gallery,
    #[not_found]
    #[at("/rust-powered-homepage/404")]
    NotFound
}

fn switch(route: Route) -> Html {
    let page = match route {
        Route::Home => html! { <crate::pages::home::Home /> },
        Route::Blog => html! { "blog" },
        Route::Gallery => html! { "gallery" },
        Route::NotFound => html! { "404" },
    };

    html! {
        <crate::pages::PageWrapper page={page} />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <crate::components::nav::NavBar />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
