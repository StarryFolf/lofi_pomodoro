use web_sys::MouseEvent;
use yew::{function_component, Html, Properties, Callback, html};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub name: String,
    pub value: String,
    pub onclick: Option<Callback<MouseEvent>>,
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = 
    {
        let props_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(props_onclick) = props_onclick.clone() {
                props_onclick.emit(event);
            }
        })
    };

    html!{
        <button name={props.name.clone()} onclick={onclick} disabled={props.disabled}>{props.value.clone()}</button>
    }
}