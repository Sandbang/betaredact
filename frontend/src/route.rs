use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{article_page::ArticlePage, home::Home};

#[derive(Clone, Routable, PartialEq)]
pub (crate) enum Route {
    #[at("/")]
    Home,
    #[at("/*path")]
    Article { path: String },
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::Article { path } => html! {<ArticlePage title = {"YAME"} author = {"Michael Serokurov"} content = {""}/> },
    }
}