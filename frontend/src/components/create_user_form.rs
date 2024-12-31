use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default)]
struct CreateUserFormData {
    email: String,
    username: String,
    password: String,
}

#[function_component(CreateUserForm)]
pub fn create_user_form() -> Html {
    let form_data = use_state(CreateUserFormData::default);

    let handle_input = {
        let form_data = form_data.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let field_name = input.name();
            let value = input.value();
            let current = (*form_data).clone();
            
            let mut updated = current.clone();
            match field_name.as_str() {
                "email" => updated.email = value,
                "username" => updated.username = value,
                "password" => updated.password = value,
                _ => (),
            }
            form_data.set(updated);
        }
    };

    let onsubmit = {
        let form_data = form_data.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let form_data = form_data.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://127.0.0.1:3000/api/users")
                    .header("Content-Type", "application/json")
                    .json(&*form_data)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(res) if res.ok() => {
                        form_data.set(CreateUserFormData::default());
                        gloo_console::log!("User created successfully");
                    }
                    Ok(res) => gloo_console::error!("Error creating user: {}", res.status()),
                    Err(err) => gloo_console::error!("Network error: {}", err.to_string()),
                }
            });
        }
    };

    html! {
        <form {onsubmit}>
            <div>
                <label for="email">{"Email:"}</label>
                <input type="email" id="email" name="email" required=true onchange={handle_input.clone()} value={form_data.email.clone()} />
            </div>
            <div>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" name="username" required=true onchange={handle_input.clone()} value={form_data.username.clone()} />
            </div>
            <div>
                <label for="password">{"Password:"}</label>
                <input type="password" id="password" name="password" required=true onchange={handle_input.clone()} value={form_data.password.clone()} />
            </div>
            <button type="submit">{"Create User"}</button>
        </form>
    }
}