use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod blog_posts;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/gallery")]
    Gallery,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(route: Route) -> Html {
    let page = match route {
        Route::Home => html! { <crate::pages::home::Home /> },
        Route::Blog => html! { <crate::pages::blog::Blog /> },
        Route::Gallery => html! { "Nothing here yet :(" },
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
