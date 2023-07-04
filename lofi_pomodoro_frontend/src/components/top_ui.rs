use web_sys::MouseEvent;
use yew::{function_component, Html, html, Callback, use_state};
use yew_hooks::use_timeout;
use crate::components::{pomodoro::components::timer::Timer, button::Button, clock::Clock, link::Link};

#[function_component(TopUI)]
pub fn top_ui() -> Html{
    let show_clock = use_state(|| false);
    let clock_visible_class = use_state(|| false);
    let timeout_time = use_state(|| 0);

    let _timeout = {
        let show_clock_state = show_clock.clone();
        use_timeout(move || {
            show_clock_state.set(!*show_clock_state)
        }, *timeout_time)
    };

    let timer_on_click = {
        let show_clock_state = show_clock.clone();
        let clock_visible_class_state = clock_visible_class.clone();
        let timeout_time_state = timeout_time.clone();
        Callback::from(move |_: MouseEvent| {
            clock_visible_class_state.set(!*clock_visible_class_state);
            if *show_clock_state {
                timeout_time_state.set(100);
            } else {
                timeout_time_state.set(0);
                show_clock_state.set(!*show_clock_state);
            }
        })
    };

    html!{
        <div class="top_ui">
            <Clock/>
            <div class="vertical">
                <div class="horizontal">
                    <Link name="linkedin_link left" img_src="static/svg/linkedin.svg" link_url="https://www.linkedin.com/in/bao-nguyen-172029261" img_class="icon shadow"/>
                    <Button name="timer_btn" value="" onclick={timer_on_click} disabled=false is_img=true img_link="static/svg/timer.svg" img_class="icon shadow"/>
                </div> 
                if *show_clock {
                    <Timer is_visible={*clock_visible_class}/>
                }
            </div>
        </div>
    }
}