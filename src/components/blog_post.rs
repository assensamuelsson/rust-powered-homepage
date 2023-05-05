use yew::prelude::*;
use chrono::NaiveDate;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub title: String,
    pub published: NaiveDate,
    pub content: Html,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    html! {
        <article class="blog-post">
            <h2>{ props.title.clone() }</h2>
            <p class="published">{ props.published.clone().format("%Y-%m-%d") }</p>
            { props.content.clone() }
        </article>
    }
}