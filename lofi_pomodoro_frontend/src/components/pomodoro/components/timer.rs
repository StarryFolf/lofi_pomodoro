use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, HtmlInputElement, KeyboardEvent, MouseEvent, InputEvent};
use yew::{function_component, html, use_state, Callback, Html, Properties};
use yew_hooks::{use_timeout, use_interval};

use crate::components::{button::Button, textbox_input::TextBoxInput};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub is_visible: bool,
}

#[function_component(Timer)]
pub fn timer(props: &Props) -> Html {
    let original_time = use_state(|| 300);
    let timer_running = use_state(|| false);
    let timer_paused = use_state(|| false);
    let time = use_state(|| 300);
    let timeout_time = use_state(|| 0);
    let interval_time = use_state(|| 0);
    let minute = use_state(|| 5);
    let second = use_state(|| 0);
    let as_input_minute = use_state(|| false);
    let as_input_second = use_state(|| false);

    let _timeout = {
        let original_time_state = original_time.clone();
        let time_state = time.clone();
        let timer_running_state = timer_running.clone();
        let timer_paused_state = timer_paused.clone();
        let timeout_time_state = timeout_time.clone();
        let timeout_time = *timeout_time_state;
        let interval_time_state = interval_time.clone();

        use_timeout(
            move || {
                timer_running_state.set(false);
                timer_paused_state.set(false);
                time_state.set(*original_time_state);
                timeout_time_state.set(0);
                interval_time_state.set(0);
            },
            timeout_time * 1000,
        )
    };

    {
        let time_state = time.clone();
        let interval_time_state = interval_time.clone();

        use_interval(move || {
            time_state.set(*time_state - 1);
        }, *interval_time_state)
    }

    let incr_minutes = {
        let minute_state = minute.clone();
        Callback::from(move |_: MouseEvent| {
            if *minute_state < 120 {
                minute_state.set(*minute_state + 1);
            }
        })
    };

    let decr_minutes = {
        let minute_state = minute.clone();
        Callback::from(move |_: MouseEvent| {
            if *minute_state > 1 {
                minute_state.set(*minute_state - 1);
            }
        })
    };

    let start_timer = {
        let minute_state = minute.clone();
        let second_state = second.clone();
        let time_state = time.clone();
        let original_time_state = original_time.clone();
        let timer_running_state = timer_running.clone();
        let timer_paused_state = timer_paused.clone();
        let timeout_time_state = timeout_time.clone();
        let interval_time_state = interval_time.clone();

        Callback::from(move |_: MouseEvent| {
            if !*timer_running_state && !*timer_paused_state {
                time_state.set(*minute_state * 60 + *second_state);
                original_time_state.set(*minute_state * 60 + *second_state);
            }
            timer_running_state.set(true);
            timer_paused_state.set(false);
            timeout_time_state.set(*time_state);
            interval_time_state.set(1000);
        })
    };

    let pause_timer = {
        let timer_running_state = timer_running.clone();
        let timer_paused_state = timer_paused.clone();
        let timeout_time_state = timeout_time.clone();
        let interval_time_state = interval_time.clone();

        Callback::from(move |_: MouseEvent| {
            timer_running_state.set(false);
            timer_paused_state.set(true);
            timeout_time_state.set(0);
            interval_time_state.set(0);
        })
    };

    let stop_timer = {
        let original_time_state = original_time.clone();
        let time_state = time.clone();
        let timer_running_state = timer_running.clone();
        let timer_paused_state = timer_paused.clone();
        let timeout_time_state = timeout_time.clone();
        let interval_time_state = interval_time.clone();

        Callback::from(move |_: MouseEvent| {
            timer_running_state.set(false);
            timer_paused_state.set(false);
            time_state.set(*original_time_state);
            timeout_time_state.set(0);
            interval_time_state.set(0);
        })
    };

    let change_second = {
        let second_state = second.clone();
        let as_input_second_state = as_input_second.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key().eq("Enter") {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<HtmlInputElement>();
                second_state.set({
                    let second_value = input.value().parse::<u32>().unwrap_or(*second_state);
                    if second_value > 59 {
                        59
                    } else {
                        second_value
                    }
                });
                as_input_second_state.set(false);
            }
        })
    };

    let change_minute = {
        let minute_state = minute.clone();
        let as_input_minute_state = as_input_minute.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key().eq("Enter") {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<HtmlInputElement>();
                minute_state.set({
                    let minute_value = input.value().parse::<u32>().unwrap_or(*minute_state);
                    if minute_value > 120 {
                        120
                    } else if minute_value == 0 {
                        1
                    } else {
                        minute_value
                    }
                });
                as_input_minute_state.set(false);
            }
        })
    };

    let handle_blur_minute = {
        let minute_state = minute.clone();
        let as_input_minute_state = as_input_minute.clone();
        Callback::from(move |event: FocusEvent| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            minute_state.set({
                let minute_value = input.value().parse::<u32>().unwrap_or(1);
                if minute_value > 120 {
                    120
                } else if minute_value == 0 {
                    1
                } else {
                    minute_value
                }
            });
            as_input_minute_state.set(false);
        })
    };

    let handle_blur_second = {
        let second_state = second.clone();
        let as_input_second_state = as_input_second.clone();
        Callback::from(move |event: FocusEvent| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            second_state.set({
                let second_value = input.value().parse::<u32>().unwrap_or(0);
                if second_value > 59 {
                    59
                } else {
                    second_value
                }
            });
            as_input_second_state.set(false);
        })
    };

    let handle_click_minute = {
        let as_input_minute_state = as_input_minute.clone();
        Callback::from(move |_: MouseEvent| {
            as_input_minute_state.set(true);
        })
    };

    let handle_click_second = {
        let as_input_second_state = as_input_second.clone();
        Callback::from(move |_: MouseEvent| {
            as_input_second_state.set(true);
        })
    };

    let resize_input = {
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            input.style().set_property("width", &(input.value().len().to_string() + "ch")).unwrap();
        })
    };

    html! {
        <div class={if !props.is_visible{
            "pomodoro fadeOut"
        } else {
            "pomodoro fadeIn"
        }}>
            <div>
                <Button name="subtract_time_btn" value="-" onclick={decr_minutes} disabled={*timer_running || *timer_paused} is_img=false img_link="" img_class=""/>
                <span class="clock_txt">
                if !*timer_running && !*timer_paused {
                    <TextBoxInput as_input={*as_input_minute} value={ if *minute < 10{
                        format!("0{}", (*minute).to_string())
                    } else {
                        (*minute).to_string()
                    } } class_name="minute_txt" on_key_press={change_minute} on_blur={handle_blur_minute} on_click={handle_click_minute} on_input={resize_input.clone()}/>
                    {":"}
                    <TextBoxInput as_input={*as_input_second} value={ if *second < 10{
                        format!("0{}", (*second).to_string())
                    } else {
                        (*second).to_string()
                    } } class_name="second_txt" on_key_press={change_second} on_blur={handle_blur_second} on_click={handle_click_second} on_input={resize_input}/>
                } else if *timer_paused || *timer_running {
                    {
                        format!("{}:{}", if (*time / 60) < 10 {
                            format!("0{}", (*time / 60).to_string())
                        } else {
                            (*time / 60).to_string()
                        },
                        if (*time % 60) < 10 {
                            format!("0{}", (*time % 60).to_string())
                        } else {
                            (*time % 60).to_string()
                        }
                        )
                    }
                }
                </span>
                <Button name="add_time_btn" value="+" onclick={incr_minutes} disabled={*timer_running || *timer_paused} is_img={false} img_link="" img_class=""/>
            </div>
            <div>
                <Button name="start_timer_btn" value={
                    if *timer_paused || *timer_running {
                        "Resume"
                    } else  {
                        "Start"
                    }
                } onclick={start_timer} disabled={*timer_running} is_img={false} img_link={""} img_class={""}/>
                <Button name="pause_timer_btn" value="Pause" onclick={pause_timer} disabled={!*timer_running} is_img=false img_link="" img_class=""/>
                <Button name="stop_timer_btn" value="Stop" onclick={stop_timer} disabled={*timer_running || !*timer_paused} is_img=false img_link="" img_class=""/>
            </div>
        </div>
    }
}
