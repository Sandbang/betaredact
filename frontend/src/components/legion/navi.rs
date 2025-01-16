
use stylist::{yew::styled_component, Style};
use yew::{html, Html};

use crate::components::cohort::bars::Bars;




const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Navi)]
pub fn navi() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <div class = {stylesheet.unwrap()}>
                <div class = "navi">
                    <a href = {"/"}><h1 class = {"center"}>{"β"} <i><sub>{"redaction"}</sub></i> </h1></a>
                </div>
            </div>
        </>
    }
}