use stylist::{yew::styled_component, Style};
use yew::{html, Html};




const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Built)]
pub fn built() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <div class={"Built"}> <p> <bdi>{"built entirely in"}</bdi> <bdi style = {"color: #d05a27"}>{" Rust "}</bdi> {"using"} {" "} <a href={"https://docs.rs/yew/latest/yew/"} target={"_blank"} >{"yew.rs,"}</a> <a href = {"https://actix.rs"} target={"_blank"}> {" actix-web"} </a> <bdi>{" and "}</bdi> <a href = {"https://www.postgresql.org"} target={"_blank"}>{"postgreSQL"}</a> </p></div>
        </>
    }
} 