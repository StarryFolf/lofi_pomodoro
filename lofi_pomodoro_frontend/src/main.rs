pub mod components;

use yew::prelude::*;
use crate::components::{top_ui::TopUI, lofi_radio::components::yt_player::YTPlayer};

#[function_component(App)]
fn app() -> Html {
    html!{
        <div class="container">
            <img class="background_gif" src="static/img/lofi-background.gif"/>
            <div class="darken"/>
            <div class="crt_lines"/>
            <div class="vignette"/>
            <TopUI />
            <YTPlayer id="5yx6BWlEVcY" width="100%" height="100%" autoplay=true/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}