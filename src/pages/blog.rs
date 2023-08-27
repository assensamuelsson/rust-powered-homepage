use yew::prelude::*;

use crate::components::blog_post::BlogPost;
use crate::blog_posts::tests_as_examples::tests_as_examples;
use crate::blog_posts::dont_mock_your_persistence_layer::dont_mock_your_persistence_layer;

#[function_component(Blog)]
pub fn blog() -> Html {
    let blog_posts = vec![
        dont_mock_your_persistence_layer(),
        tests_as_examples(),
    ];

    blog_posts.iter().map(|p| html! { <BlogPost title={p.title.clone()} published={p.published.clone()} content={p.content.clone()} /> }).collect::<Html>()
}