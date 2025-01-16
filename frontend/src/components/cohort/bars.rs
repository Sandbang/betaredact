
use stylist::{yew::styled_component, Style};
use yew::{html, Html};




const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Bars)]
pub fn bars() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            <div>
                <img class = {"settings"} src = {"assets/menuBar.jpg"} alt={"img"} width = {"40px"}/>
            </div>
        </>
    }
}