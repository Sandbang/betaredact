use stylist::{yew::styled_component, Style};
use yew::{html, Html};




const STYLE_FILE: &str = include_str!("../../main.css");
#[styled_component(Built)]
pub fn built() -> Html {
    let stylesheet = Style::new(STYLE_FILE);
    html!{
        <>
            
        </>
    }
} 