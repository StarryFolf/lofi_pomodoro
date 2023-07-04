use serde::Serialize;
use wasm_bindgen::{JsValue, JsCast, prelude::{Closure, wasm_bindgen}};
use web_sys::{window, HtmlScriptElement, Event};

pub async fn load_iframe_api_async() -> Result<JsValue, JsValue> {
    let window = window().unwrap();
    let document = window.document().ok_or("Document is not defined")?;
    let script = document.create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;

    script.set_src("https://www.youtube.com/iframe_api");

    let (sender, receiver) = futures::channel::oneshot::channel();
    let onload_callback = Closure::once(move || {
        let _ = sender.send(Ok(JsValue::TRUE));
    });

    script.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    onload_callback.forget();

    document.body().ok_or("No head tag found")?.append_child(&script)?;

    receiver.await.unwrap()
}

pub async fn yt_api_ready_async() -> Result<JsValue, JsValue> {
    let window = window().ok_or("Window is not defined")?;
    let yt = js_sys::Reflect::get(&window, &"YT".into())?;

    if !yt.is_undefined() && js_sys::Reflect::has(&yt, &"Player".into())? {
        Ok(yt)
    } else {
        load_iframe_api_async().await?;
        let (sender, receiver) = futures::channel::oneshot::channel();
        let resolve_closure = Closure::once(move || {
            let _: Result<(), Result<JsValue, Option<JsValue>>> = sender.send(Ok(JsValue::TRUE));
        });

        js_sys::Reflect::set(
            &window,
            &"onYouTubeIframeAPIReady".into(),
            &resolve_closure.into_js_value(),
        )?;

        let _ = receiver
            .await
            .map_err(|e| format!("failed to await the youtube iframe api: {}", e.to_string()))?;

        Ok(js_sys::Reflect::get(&window, &"YT".into())?)
    }
}

#[derive(Serialize, Default)]
pub struct PlayerVars {
    #[serde(rename = "autoplay", skip_serializing_if = "Option::is_none")]
    pub autoplay: Option<u8>,
    #[serde(rename = "controls", skip_serializing_if = "Option::is_none")]
    pub controls: Option<u8>,
    #[serde(rename = "enablejsapi", skip_serializing_if = "Option::is_none")]
    pub enable_js_api: Option<u8>,
    #[serde(rename = "fs", skip_serializing_if = "Option::is_none")]
    pub full_screen: Option<u8>,
    #[serde(rename = "iv_load_policy", skip_serializing_if = "Option::is_none")]
    pub iv_load_policy: Option<u8>,
    #[serde(rename = "modestbranding", skip_serializing_if = "Option::is_none")]
    pub modest_branding: Option<u8>,
    #[serde(rename = "playsinline", skip_serializing_if = "Option::is_none")]
    pub plays_inline: Option<u8>,
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub related_videos: Option<u8>,
    #[serde(rename = "showinfo", skip_serializing_if = "Option::is_none")]
    pub show_info: Option<u8>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<u32>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "widget_referrer", skip_serializing_if = "Option::is_none")]
    pub widget_referrer: Option<String>,
}

#[derive(Serialize, Default)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(rename = "videoId", skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[serde(rename = "playerVars", skip_serializing_if = "Option::is_none")]
    pub player_vars: Option<PlayerVars>,
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object ,js_namespace = YT, js_name = Player , typescript_type = "Player")]
    #[derive(PartialEq, Clone, Eq)]
    pub type Player;

    #[wasm_bindgen(js_namespace = YT, js_class="Player", constructor)]
    pub fn new(target_id: &str, options: JsValue) -> Player;

    #[wasm_bindgen(catch, method, structural, js_name=playVideo)]
    pub fn play_video(this: &Player) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, structural, js_name=pauseVideo)]
    pub fn pause_video(this: &Player) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name=stopVideo)]
    pub fn stop_video(this: &Player);

    #[wasm_bindgen(method, js_name=cueVideoById)]
    pub fn cue_video_by_id(this: &Player, video_id: JsValue);

    #[wasm_bindgen(method, js_name=addEventListener)]
    pub fn add_event_listener(this: &Player, event: &str, callback: &Closure<dyn FnMut(Event)>);

    #[wasm_bindgen(method, js_name=removeEventListener)]
    pub fn remove_event_listener(
        this: &Player,
        event: &str,
        callback: &Closure<dyn FnMut(Event)>,
    );

    #[wasm_bindgen(method, js_name=getPlayerState)]
    pub fn get_player_state(this: &Player) -> i32;
}