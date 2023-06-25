use js_sys::Date;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use yew::{function_component, Html, use_state, html};
use yew_hooks::use_interval;

#[derive(Serialize, Deserialize)] 
struct Options {
    year: String,
    month: String,
    day: String,
    weekday: String,
    hour: String,
    minute: String,
    second: String
}

#[wasm_bindgen]
pub fn make_options() -> JsValue{
    let options = Options {
        year: "numeric".to_string(),
        month: "short".to_string(),
        day: "2-digit".to_string(),
        weekday: "short".to_string(),
        hour: "2-digit".to_string(),
        minute: "2-digit".to_string(),
        second: "2-digit".to_string(),
    };

    serde_wasm_bindgen::to_value(&options).unwrap()
}

fn get_current_date_time() -> String {
    let options = make_options();
    let date_time = Date::new_0();
    String::from(date_time.to_locale_string("en-US", &options))
}

#[function_component(Clock)]
pub fn clock() -> Html {
    let current_date_time = use_state(get_current_date_time);

    {
        let current_date_time_state = current_date_time.clone();

        use_interval(move || {
            current_date_time_state.set(get_current_date_time());
        }, 1000);
    }

    html! {
        <div class="curr_date_time">
            <span> {current_date_time.as_str()} </span>
        </div>
    }
}