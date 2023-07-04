use yew::{Properties, Html, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(BottomUI)]
pub fn bottom_ui(props: &Props) -> Html {
    html!{
        <div class="bottom_ui">
            <span>{props.title.clone()}</span>
        </div>
    }
}