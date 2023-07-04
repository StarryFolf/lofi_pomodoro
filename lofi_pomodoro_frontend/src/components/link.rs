use yew::{function_component, Html, Properties, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub link_url: String,
    pub img_src: String,
    pub name: String,
    pub img_class: String,
}

#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    html! {
        <a href={props.link_url.clone()} class={props.name.clone()} target="_blank">
            <img src={props.img_src.clone()} class={props.img_class.clone()}/>
        </a>
    }
}