use std::rc::Rc;

use futures::FutureExt;
use wasm_bindgen::{prelude::Closure, JsValue};
use yew::{function_component, Properties, Html, html, platform::spawn_local};

use crate::components::lofi_radio::imports::yt::{Player, Options, PlayerVars, yt_api_ready_async};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub width: String,
    pub height: String,
    pub autoplay: bool,

}

#[function_component(YTPlayer)]
pub fn yt_player(props: &Props) -> Html {
    let id = props.id.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    let autoplay = props.autoplay;
    spawn_local(yt_api_ready_async().map(move |_| {
        let player = Rc::new(Player::new(
            "yt_player",
            serde_wasm_bindgen::to_value(&Options {
                height: Some(height),
                width: Some(width),
                video_id: Some(id),
                player_vars: Some(PlayerVars {
                    autoplay: if autoplay {Some(1)} else {Some(0)},
                    ..Default::default()
                })
            }).unwrap()
        ));

        let player_clone = player.clone();
        let closure = Closure::once(move |_: JsValue| {
            player_clone.play_video();
        });
        player.add_event_listener("onReady", &closure);
        closure.forget();
    }));

    html!{
        <div class="yt_wrapper">
            <div style="width: 100%; height: 100%; overflow: hidden; display: flex; align-items: center; justify-content: center; border-radius: 8px;">
                <div class="disable_pointer">
                    <div style="width: 100%; height: 100%">
                        <div id="yt_player"/>
                    </div>
                </div>
            </div>
        </div>
    }
}