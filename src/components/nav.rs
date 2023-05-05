use yew::prelude::*;

use crate::components::logo::Logo;

#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    text: String,
    href: String,
}

#[function_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {
    html! {
        <a class="nav-item" href={ props.href.clone() }>{ props.text.clone() }</a>
    }
}


#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <header>
            <div class="horizontal-clamp">
                <a href="/"><Logo /></a>
                <input class="side-menu" type="checkbox" id="side-menu" />
                <label class="hamburger" for="side-menu"><span class="hamburger-line"/></label>
                <nav>
                    <ul class="menu">
                        <li><NavItem href="/rust-powered-homepage" text="Home" /></li>
                        <li><NavItem href="/rust-powered-homepage/blog" text="Blog" /></li>
                        <li><NavItem href="/rust-powered-homepage/gallery" text="Gallery" /></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}

