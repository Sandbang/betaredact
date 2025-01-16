mod pages;
mod components;
mod route;
use pages::home::Home;
use stylist::Style;
use yew::Html;
use yew::prelude::*;
use stylist::yew::styled_component;
use route::switch;
use yew_router::BrowserRouter;
use yew_router::Switch;
use route::Route;

const STYLE_FILE: &str = include_str!("main.css");
#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <BrowserRouter> 
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}   