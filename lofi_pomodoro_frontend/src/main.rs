pub mod pomodoro;

use yew::prelude::*;
use crate::pomodoro::components::clock::Clock;

#[function_component(App)]
fn app() -> Html {
    html!{
        <Clock />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
