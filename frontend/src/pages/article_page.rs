use stylist::{yew::styled_component, Style};
use yew::{html, Html, Properties};
use crate::components::centuries::built::Built;
use crate::components::legion::navi::Navi;

#[derive(Properties, PartialEq)]pub struct Props {
    pub title: String,
    pub author: String,
    pub content: String,
}


const STYLE_FILE: &str = include_str!("../main.css");
#[styled_component(ArticlePage)]
pub fn article_page(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{ 
        <body  class = {stylesheet.unwrap()}>
                <Navi />
                <Built />
            <div class = {"content"}>
                <h2>{&props.title}</h2>
                <p>{&props.content}</p>
                <h3>{&props.author}</h3>
            </div>
        </body>
    }
} 