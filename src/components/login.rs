use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-full bg-gray-900 text-white flex items-center justify-center p-4">
            <div class="bg-gray-800 rounded-xl shadow-lg p-8 w-full max-w-md space-y-6">
                <h2 class="text-2xl font-semibold text-center text-violet-400">{"Welcome Back 👋"}</h2>
                <form class="space-y-4">
                    <input
                        {oninput}
                        placeholder="Enter your username"
                        class="
                            w-full px-4 py-3 rounded-lg
                            bg-gray-700 text-white placeholder-gray-400
                            border border-gray-600 focus:outline-none
                            focus:ring-2 focus:ring-violet-500 focus:border-transparent
                            transition duration-150 ease-in-out
                        "
                    />
                    <Link<Route> to={Route::Chat} classes="block">
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="
                                w-full py-3 px-4 rounded-lg
                                bg-violet-600 hover:bg-violet-700
                                disabled:opacity-50 disabled:cursor-not-allowed
                                text-white font-bold uppercase tracking-wide
                                transition-all duration-150
                                focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500
                            "
                        >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}