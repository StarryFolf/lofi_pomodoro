use futures::FutureExt;
use gloo_console::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, KeyboardEvent, Event};
use yew::{function_component, html, platform::spawn_local, Callback, Html, Properties, use_state};

use crate::components::lofi_radio::imports::yt::{yt_api_ready_async, Options, Player, PlayerVars};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: String,
    pub width: String,
    pub height: String,
    pub autoplay: bool,
    pub title_callback: Callback<String>,
}

#[function_component(YTPlayer)]
pub fn yt_player(props: &Props) -> Html {
    let id = props.id.clone();
    let width = props.width.clone();
    let height = props.height.clone();
    let autoplay = props.autoplay;
    let player_state = use_state(|| -1);
    {
        let player_state = player_state.clone();
        let callback_clone = props.title_callback.clone(); 
        spawn_local(yt_api_ready_async().map(move |_| {
            let player = Player::new(
                "yt_player",
                serde_wasm_bindgen::to_value(&Options {
                    height: Some(height),
                    width: Some(width),
                    video_id: Some(id),
                    player_vars: Some(PlayerVars {
                        autoplay: if autoplay { Some(1) } else { Some(0) },
                        ..Default::default()
                    }),
                })
                .unwrap(),
            );
            let closure = Closure::once(move |event: Event| {
                let document = window().unwrap().document().unwrap();
                let element = document.get_element_by_id("yt_player").unwrap();
                let title = element.get_attribute("title").unwrap();
                let player = event.target().unwrap().unchecked_into::<Player>();
                player.play_video().unwrap();
                callback_clone.emit(title);
            });
            player.add_event_listener("onReady", &closure);
            closure.forget();
            let key_down_handler: Closure<dyn FnMut(KeyboardEvent)> = Closure::new(move |event: KeyboardEvent| {
                if event.code().eq("Space") && !js_sys::Reflect::get(player.as_ref(), &"u".into()).unwrap().is_null(){
                    if *player_state != 1 {
                        player.play_video().unwrap();
                        player_state.set(1);
                    } else {
                        player.pause_video().unwrap();
                        player_state.set(2);
                    }
                }
            });
            let document = window().unwrap().document().unwrap();
            document.add_event_listener_with_callback("keydown", key_down_handler.as_ref().unchecked_ref()).unwrap();
            key_down_handler.forget();
        }));
    }

    html! {
        <div class="yt_wrapper">
            <div style="width: 100%; height: 100%; overflow: hidden; display: flex; align-items: center; justify-content: center; border-radius: 8px;">
                <div class="disable_pointer">
                    <div style="width: 100%; height: 104%">
                        <div id="yt_player"/>
                    </div>
                </div>
            </div>
        </div>
    }
}
