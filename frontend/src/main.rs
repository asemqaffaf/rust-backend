mod components;

use components::counter::Counter;
use components::create_user_form::CreateUserForm;
use components::user_details::UserDetails;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let user_id = use_state(|| "".to_string());

    let get_user_id = {
        let user_id = user_id.clone();
        Callback::from(move |id: String| {
            user_id.set(id);
        })
    };

    html! {
        <div>
            <h1>{"User Management"}</h1>
            <CreateUserForm on_user_created={get_user_id} />
            <UserDetails id={user_id.to_string()} />
            <Counter />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
