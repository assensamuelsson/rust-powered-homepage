use yew::prelude::*;

pub mod home;

#[derive(Properties, PartialEq)]
pub struct PageWrapperProps {
    pub page: Html,
}

#[function_component(PageWrapper)]
pub fn page_wrapper(props: &PageWrapperProps) -> Html {
    html! {
        <main class="horizontal-clamp">
            { props.page.clone() }
        </main>
    }
}
