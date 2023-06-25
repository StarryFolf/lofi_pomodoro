use web_sys::{FocusEvent, MouseEvent, KeyboardEvent, HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Html, Properties, use_node_ref, use_effect};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub as_input: bool,
    pub value: String,
    pub class_name: String,
    pub on_key_press: Option<Callback<KeyboardEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_click: Option<Callback<MouseEvent>>,
    pub on_input: Option<Callback<InputEvent>>
}

#[function_component(TextBoxInput)]
pub fn textbox_input(props: &Props) -> Html {
    let on_key_press = {
        let prop_on_key_press = props.on_key_press.clone();
        Callback::from(move |event: KeyboardEvent| {
            if let Some(prop_on_key_press) = prop_on_key_press.clone() {
                prop_on_key_press.emit(event)
            }
        })
    };

    let on_blur = {
        let prop_on_blur = props.on_blur.clone();
        Callback::from(move |event: FocusEvent| {
            if let Some(prop_on_blur) = prop_on_blur.clone() {
                prop_on_blur.emit(event)
            }
        })
    };

    let on_click = {
        let prop_on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(prop_on_click) = prop_on_click.clone() {
                prop_on_click.emit(event)
            }
        })
    };

    let on_input = {
        let prop_on_input = props.on_input.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(prop_on_input) = prop_on_input.clone() {
                prop_on_input.emit(event)
            }
        })
    };

    let input_element = use_node_ref();

    {
        let input_element_clone = input_element.clone();
        let as_input = props.as_input;
        use_effect(move || {
            if as_input {
                let input = input_element_clone.cast::<HtmlInputElement>().unwrap();
                input.focus().unwrap();
                input.style().set_property("width", &(input.value().len().to_string() + "ch")).unwrap();
            }
        });
    }

    html! {
        <span class={props.class_name.clone()} >
            if props.as_input {
                <input
                    type="text"
                    value={props.value.clone()}
                    onkeypress={on_key_press}
                    onblur={on_blur}
                    oninput={on_input}
                    ref={input_element}
                />
            } else {
                <span onclick={on_click}>
                    {props.value.clone()}
                </span>
            }
        </span>
    }
}
