use stylist::{yew::styled_component, Style};
use yew::{html, Html};
use crate::components::centuries::built::Built;
use crate::components::legion::navi::Navi;
use crate::components::cohort::post::Post;



const STYLE_FILE: &str = include_str!("../main.css");
#[styled_component(Home)]
pub fn home() -> Html {
    let title = "Lorem Ipsum";
    let description = "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness";
    let author = "Michael Serokurov";
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <body  class = {stylesheet.unwrap()}>
            <div><Navi /></div>
            <Built />
            <ul>
                <li><Post title = {title} description = {description} author = {author} /></li>
                <li><Post title = {title} description = {description} author = {author}/></li>
            </ul>
        </body>
    }
} 