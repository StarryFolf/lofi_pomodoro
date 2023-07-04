pub mod components;
use yew::prelude::*;
use crate::components::{top_ui::TopUI, lofi_radio::components::yt_player::YTPlayer, bottom_ui::BottomUI};

#[function_component(App)]
fn app() -> Html {
    let title = use_state(|| String::new());
    let title_callback = 
    {
        let title_state = title.clone();
        Callback::from(move |title: String| {
            title_state.set(title);
        })
    };

    html!{
        <div class="container">
            <img class="background_gif" src="static/img/lofi-background.gif"/>
            <div class="darken"/>
            <div class="crt_lines"/>
            <div class="vignette"/>
            <TopUI />
            <YTPlayer id="5yx6BWlEVcY" width="100%" height="100%" autoplay=true {title_callback}/>
            <BottomUI title={(*title).clone()}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}