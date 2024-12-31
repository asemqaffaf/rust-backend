use yew::prelude::*;
// use yew::format::Nothing;
// use yew::services::fetch::Request;
// use gloo_net::http::Request;

#[function_component]
pub fn Counter() -> Html {


    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="min-h-screen bg-gray-100">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-4xl font-bold text-center text-gray-800 mb-8">
                    { "Welcome to Yew App" }
                </h1>
                <div class="max-w-md mx-auto bg-white rounded-lg shadow-lg p-6">
                    <button
                        {onclick}
                        class="w-full bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition duration-200 ease-in-out transform hover:scale-105"
                    >
                        { "Click me!" }
                    </button>
                    <p class="mt-4 text-center text-gray-700 text-lg">
                        { "Counter value: " }
                        <span class="font-bold text-blue-600">{ *counter }</span>
                    </p>
                </div>
            </div>
        </div>
    }
}

// fn main() {
//     yew::Renderer::<GetUser>::new().render();
// }