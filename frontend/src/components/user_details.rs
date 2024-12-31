use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserDetailsProps {
    pub id: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct UserData {
    email: String,
    id: String,
    created_at: String,
    updated_at: String,
    username: String,
}


#[function_component(UserDetails)]
pub fn user_details(props: &UserDetailsProps) -> Html {
    let user_data = use_state(|| None::<UserData>);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    {
        let id = props.id.clone();
        let user_data = user_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                if !id.is_empty() {
                    loading.set(true);
                    error.set(None);

                    spawn_local(async move {
                        match fetch_user_data(&id).await {
                            Ok(data) => {
                                user_data.set(Some(data));
                                loading.set(false);
                            }
                            Err(err) => {
                                error.set(Some(err.to_string()));
                                loading.set(false);
                            }
                        }
                    });
                }
                || ()
            },
            props.id.clone(),
        );
    }

    html! {
        <div class="mt-8 p-4 bg-white rounded shadow">
            <h2 class="text-2xl font-bold mb-4">{"User Details"}</h2>
            if props.id.is_empty() {
                <p class="text-gray-600">{"Please create a user to see details"}</p>
            } else if *loading {
                <p class="text-blue-600">{"Loading..."}</p>
            } else if let Some(err) = error.as_ref() {
                <p class="text-red-600">{format!("Error: {}", err)}</p>
            } else if let Some(data) = (*user_data).as_ref() {
                <div class="space-y-2">
                    <p class="text-gray-800"><strong>{"Email: "}</strong>{&data.email}</p>
                    <p class="text-gray-800"><strong>{"Username: "}</strong>{&data.username}</p>
                    <p class="text-gray-800"><strong>{"Created: "}</strong>{&data.created_at}</p>
                    <p class="text-gray-800"><strong>{"Updated: "}</strong>{&data.updated_at}</p>
                </div>
            }
        </div>
    }
}

async fn fetch_user_data(id: &str) -> Result<UserData, String> {
    let url = format!("http://localhost:3000/api/users/{}", id);

    match Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<UserData>()
        .await
    {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}