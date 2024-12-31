mod components;

use components::create_user_form::CreateUserForm;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"User Management"}</h1>
            <CreateUserForm />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
