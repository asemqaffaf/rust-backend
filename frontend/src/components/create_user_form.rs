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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_user_created: Callback<String>,
}

#[function_component(CreateUserForm)]
pub fn create_user_form(props: &Props) -> Html {
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
        let on_user_created = props.on_user_created.clone();
        
        move |e: SubmitEvent| {
            e.prevent_default();
            let form_data = form_data.clone();
            let on_user_created = on_user_created.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://127.0.0.1:3000/api/users")
                    .header("Content-Type", "application/json")
                    .json(&*form_data)
                    .unwrap()
                    .send()
                    .await;

                    
                match response {
                    Ok(res) if res.ok() => {
                        if let Ok(user_data) = res.json::<serde_json::Value>().await {
                            if let Some(id) = user_data.get("id").and_then(|v| v.as_str()) {
                                on_user_created.emit(id.to_string());
                            }
                            form_data.set(CreateUserFormData::default());
                            gloo_console::log!("User created successfully");
                        }
                    },
                    Ok(res) => gloo_console::error!("Error creating user: {}", res.status()),
                    Err(err) => gloo_console::error!("Network error: {}", err.to_string()),
                }
            });
        }
    };

    html! {
        <form {onsubmit} class="space-y-4">
            <div>
                <label for="email" class="block text-gray-700">{"Email:"}</label>
                <input 
                    type="email" 
                    id="email" 
                    name="email" 
                    required=true 
                    onchange={handle_input.clone()} 
                    value={form_data.email.clone()}
                    class="mt-1 p-2 border border-gray-300 rounded-md w-full focus:ring-indigo-500 focus:border-indigo-500" 
                />
            </div>
            <div>
                <label for="username" class="block text-gray-700">{"Username:"}</label>
                <input 
                    type="text" 
                    id="username" 
                    name="username" 
                    required=true 
                    onchange={handle_input.clone()} 
                    value={form_data.username.clone()}
                    class="mt-1 p-2 border border-gray-300 rounded-md w-full focus:ring-indigo-500 focus:border-indigo-500" 
                />
            </div>
            <div>
                <label for="password" class="block text-gray-700">{"Password:"}</label>
                <input 
                    type="password" 
                    id="password" 
                    name="password" 
                    required=true 
                    onchange={handle_input.clone()} 
                    value={form_data.password.clone()}
                    class="mt-1 p-2 border border-gray-300 rounded-md w-full focus:ring-indigo-500 focus:border-indigo-500" 
                />
            </div>
            <button 
                type="submit" 
                class="mt-4 bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50"
            >
                {"Create User"}
            </button>
        </form>
    }
}