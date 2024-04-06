use countries::COUNTRY_CODES;

use web_sys::HtmlInputElement;
use yew::{function_component, Html, html, Properties, use_state, use_node_ref, UseStateHandle, Callback, NodeRef};

use super::countries;

#[derive(Properties, PartialEq)]
pub struct Props {
    // Input can be password, textarea, text, tel
    pub input_type: Option<String>,

    // The label to be displayed for the input field.
    pub label: Option<String>,

    // The name of the input field, used for form submission and accessibility
    pub name: Option<String>,

    // Indicates whether the input is required or not
    pub required: Option<bool>,

    // A reference to the DOM node of the input element
    pub input_ref: NodeRef,

    // The error message to display when there is a validation error
    pub error_message: Option<String>,

    // The CSS class to be applied to all inner elements
    pub form_input_class: Option<String>,
    
    // The CSS class to be applied to the inner input element and icon
    pub form_input_field_class: Option<String>,

    // The CSS class to be applied to the label for the input element
    pub form_input_label_class: Option<String>,

    // The CSS class to be applied to the input element
    pub form_input_input_class: Option<String>,

    // The CSS class to be applied to the error div element
    pub form_input_error_class: Option<String>,

    // The CSS class to be applied to the icon element
    pub icon_class: Option<String>,

    // The state handle for managing the value of the input
    pub input_handle: UseStateHandle<String>,

    // The state handle for managing the validity state of the input
    pub input_valid_handle: UseStateHandle<bool>,

    // A callback function to validate the input value. It takes a `String` as input and returns a `bool`
    pub validate_function: Callback<String, bool>,
    pub compare_to: Option<String>,
    
    // The icon when the password is visible
    pub eye_active: Option<String>,

    // The icon when the passowrd is not visible
    pub eye_disabled: Option<String>,



    // Additional props for accessibility and SEO:

    // The ID attribute of the input element
    pub input_id: Option<String>,

    // The placeholder text to be displayed in the input element.
    pub input_placeholder: Option<String>,

    // The aria-label attribute for screen readers, providing a label for accessibility
    pub aria_label: Option<String>,

    // The aria-required attribute for screen readers, indicating whether the input is required
    pub aria_required: Option<String>,

    // The aria-invalid attribute for screen readers, describing the input element's error message
    pub aria_invalid: Option<String>,

    // The aria-describedly attribute for screen readers, describing the input element's error message
    pub aria_describedby: Option<String>,
}


#[function_component(CustomInput)]
pub fn custom_input(props: &Props) -> Html {
    let eye_active_handle = use_state(|| false);
    let eye_active = (*eye_active_handle).clone();

    let input_country_ref = use_node_ref();
    let country_handle = use_state(|| String::default());
    let country = (*country_handle).clone();

    let password_type_handle = use_state(|| "password");
    let password_type = (*password_type_handle).clone();

    let input_valid = *props.input_valid_handle;

    let aria_invalid = props.aria_invalid.clone().unwrap_or_else(|| "true".to_owned());
    let eye_icon_active = props.eye_active.clone().unwrap_or_else(|| "fa fa-eye".to_owned());
    let eye_icon_disabled = props.eye_disabled.clone().unwrap_or_else(|| "fa fa-eye-slash".to_owned());
    let aria_required = props.aria_required.clone().unwrap_or_else(|| "true".to_owned());
    let input_type = props.input_type.clone().unwrap_or_else(|| "text".to_owned());

    let onchange = {
        let input_ref = props.input_ref.clone();
        let input_handle = props.input_handle.clone();
        let input_valid_handle = props.input_valid_handle.clone();
        let validate_function = props.validate_function.clone();
        let compare_to = props.compare_to.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                input_handle.set(value);

                match &compare_to {
                    Some(st) => {
                        if &input.value() == st {
                        input_valid_handle.set(true)
                        } else {
                            input_valid_handle.set(false)
                        }
                    },
                    None => {
                        input_valid_handle.set(validate_function.emit(input.value()));
                    }
                }

                
            }
        })
    };

    let on_select_change = {
        let input_country_ref = input_country_ref.clone();
        let input_handle = props.input_handle.clone();
        let country_handle = country_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = input_country_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                country_handle.set(value);
                input_handle.set(input.value());
            }
        })
    };

    let on_phone_number_input = {
        let input_ref = props.input_ref.clone();
        let input_handle = props.input_handle.clone();
        let country_handle = country_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                for (code, _, _, _, _, _) in &COUNTRY_CODES {
                    if code.starts_with(&input.value()) {
                        country_handle.set(input.value());
                        break;
                    }
                }
                // Filter out non-numeric characters
                let numeric_value: String =
                    input.value().chars().filter(|c| c.is_numeric()).collect();
                input_handle.set('+'.to_string() + &numeric_value);
            }
        })
    };

    let on_toggle_password = {
        Callback::from(move |_| {
            if eye_active {
                password_type_handle.set("password".into())
            } else {
                password_type_handle.set("text".into())
            }
            eye_active_handle.set(!eye_active);
        })
    };

    let input_tag = match (*input_type).into() {
        "email" => html! {
            <>  
                <div class="input-group-prepend">
                    <span class="input-group-text h-100">
                        <i class="fa fa-envelope"></i>
                    </span>
                </div>

                <input
                    class={props.form_input_input_class.clone()}
                    id={props.input_id.clone()}
                    name={props.name.clone()}
                    ref={props.input_ref.clone()}
                    placeholder={props.input_placeholder.clone()}
                    aria-label={props.aria_label.clone()}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    aria-describedby={props.aria_describedby.clone()}
                    oninput={onchange}
                    required={props.required.is_some()}
                />
            </>
        },
        
        "password" => html! {
            <>
                <div class="input-group-prepend">
                    <span class="input-group-text h-100">
                        <i class="fa fa-lock fa-lg"></i>
                    </span>
                </div>

                <input
                    type={password_type}
                    class={props.form_input_input_class.clone()}
                    id={props.input_id.clone()}
                    name={props.name.clone()}
                    ref={props.input_ref.clone()}
                    placeholder={props.input_placeholder.clone()}
                    aria-label={props.aria_label.clone()}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    aria-describedby={props.aria_describedby.clone()}
                    oninput={onchange}
                    required={props.required.is_some()}
                />
                
                <div class="input-group-append">
                    <span class="input-group-text h-100" onclick={on_toggle_password}>
                        <i class={
                            format!("toggle-button {}", if eye_active { eye_icon_active } else { eye_icon_disabled })}   
                        ></i>
                    </span>   
                </div>
            </>
        },

        "password_again" => html! {
            <>
                <div class="input-group-prepend">
                    <span class="input-group-text h-100">
                        <i class="fa fa-lock fa-lg"></i>
                    </span>
                </div>

                <input
                    type={password_type}
                    class={props.form_input_input_class.clone()}
                    id={props.input_id.clone()}
                    name={props.name.clone()}
                    ref={props.input_ref.clone()}
                    placeholder={props.input_placeholder.clone()}
                    aria-label={props.aria_label.clone()}
                    aria-required={aria_required}
                    aria-invalid={aria_invalid}
                    aria-describedby={props.aria_describedby.clone()}
                    oninput={onchange}
                    required={props.required.is_some()}
                />
            </>
        },

        "textarea" => html! {
            <textarea
                class={props.form_input_input_class.clone()}
                id={props.input_id.clone()}
                name={props.name.clone()}
                ref={props.input_ref.clone()}
                placeholder={props.input_placeholder.clone()}
                aria-label={props.aria_label.clone()}
                aria-required={aria_required}
                aria-invalid={aria_invalid}
                aria-describedby={props.aria_describedby.clone()}
                oninput={onchange}
                required={props.required.is_some()}
            >
            </textarea>
        },
        "tel" => html! {
                <>
                    <select
                        ref={input_country_ref}
                        onchange={on_select_change}
                    >
                        { for COUNTRY_CODES.iter().map(|(code, emoji, _, name, _, _)| {
                            let selected = if *code == country { true } else { false };
                            html! {
                                <option value={*code} selected={selected}>{ format!("{} {} {}", emoji, name, code) }</option>
                            }
                        })}
                    </select>
                    <input
                        type="tel"
                        id="telNo"
                        name="telNo"
                        size="20"
                        minlength="9"
                        value={(*props.input_handle).clone()}
                        maxlength="14"
                        class={props.form_input_input_class.clone()}
                        placeholder={props.input_placeholder.clone()}
                        aria-label={props.aria_label.clone()}
                        aria-required={aria_required}
                        aria-invalid={aria_invalid}
                        oninput={on_phone_number_input}
                        ref={props.input_ref.clone()}
                    />
                </>
        },
        _ => html! {
            <input
                type={input_type}
                class={props.form_input_input_class.clone()}
                id={props.input_id.clone()}
                name={props.name.clone()}
                ref={props.input_ref.clone()}
                placeholder={props.input_placeholder.clone()}
                aria-label={props.aria_label.clone()}
                aria-required={aria_required}
                aria-invalid={aria_invalid}
                aria-describedby={props.aria_describedby.clone()}
                oninput={onchange}
                required={props.required.is_some()}
            />
        },
    };
    



    html! {
        <div class={props.form_input_class.clone()}>
            <label class={props.form_input_label_class.clone()} for={props.input_id.clone()}>
                {
                    match props.label.clone() {
                        Some(value) => {value},
                        None => "".into(),
                    }
                }
            </label>
            
            <div class={props.form_input_field_class.clone()}>
                {input_tag}
            </div>
            
            if !input_valid {
                <div class={props.form_input_error_class.clone()}
                    id={props.aria_describedby.clone()}
                    >{&props.error_message.clone().unwrap()}
                </div>
            }
        </div>
    }
}