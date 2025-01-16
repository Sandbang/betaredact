use stylist::{yew::styled_component, Style};
use yew::{html, Html};

use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub description: String,
    pub author: String,
}


const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Post)]
pub fn post(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    let link = &props.title.replace(" ", "-");
    html!{
        <>
            <a href = {link.clone()}>
                <div class = {"article"}>
                <div>
                    <h2 >{&props.title}</h2>
                    <p>{&props.description}</p>
                    <h3>{&props.author}</h3>
                </div>
                </div>
            </a>
        </>
    }
} 