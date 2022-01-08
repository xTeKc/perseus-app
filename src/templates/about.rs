use perseus::Template;
use sycamore::prelude::{component, view, Html, SsrNode, View};

#[perseus::template(AboutPage)]
#[component(AboutPage<G>)]
pub fn about_page() -> View<G> {
    view! {
        p { "About." }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About Page | Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}