use gloo_console::{log};
use web_sys::MouseEvent;
use yew::{function_component, Html, use_state, Callback, html};
use yew_agent::{use_bridge, UseBridgeHandle};
use yew_hooks::{use_timeout, use_interval};

use crate::pomodoro::components::button::Button;

#[function_component(Clock)]
pub fn clock() -> Html {
    let original_time = 300;
    let timer_running = use_state(|| false);
    let time = use_state(|| 300);
    let timeout_time_state = use_state(|| 0);
    let interval_time_state = use_state(|| 0);

    let _timeout = {
        let time_state = time.clone();
        let timer_running_state = timer_running.clone();
        let timeout_time_state = timeout_time_state.clone();
        let timeout_time = *timeout_time_state;
        let interval_time_state = interval_time_state.clone();

        use_timeout(move || {
            timer_running_state.set(false);
            time_state.set(original_time);
            timeout_time_state.set(0);
            interval_time_state.set(0);
            log!("done");
        }, timeout_time * 1000)
    };

    {
        let time_state = time.clone();
        let interval_time_state = interval_time_state.clone();

        use_interval(move || {
            time_state.set(*time_state - 1);
            log!("tick");
        }, *interval_time_state * 1000);
    }

    let incr_minutes = {
        let time_state = time.clone();
        Callback::from(move |_: MouseEvent| {
            if *time_state < 7200 {
                time_state.set(*time_state + 60);
            }
        })
    };
    
    let decr_minutes = {
        let time_state = time.clone();
        Callback::from(move |_: MouseEvent| {
            if *time_state > 60 {
                time_state.set(*time_state - 60);
            }
        })
    };

    let minutes = {
        if *time / 60 < 10 {
            format!("0{}", *time / 60)
        } else {
            format!("{}", *time / 60)
        }
    };

    let seconds = {
        if *time % 60 < 10 {
            format!("0{}", *time % 60)
        } else {
            format!("{}", *time % 60)
        }
    };
    
    let start_clock = {
        let time_state = time.clone();
        let timer_running_state = timer_running.clone();

        Callback::from(move |_: MouseEvent| {
            timer_running_state.set(true);
            timeout_time_state.set(*time_state);
            interval_time_state.set(1);
        })
    };

    html! {
        <div>
            <div>
                <Button name="remove_time_btn" value="-" onclick={decr_minutes} disabled={*timer_running}/>
                <span>{format!("{}:{}", minutes, seconds)}</span>
                <Button name="add_time_btn" value="+" onclick={incr_minutes} disabled={*timer_running}/>
            </div>
            <div><Button name="start_clock_btn" value="Start" onclick={start_clock} disabled={*timer_running}/></div>
        </div>
    }
}