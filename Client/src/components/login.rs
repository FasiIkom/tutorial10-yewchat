use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;
use crate::Theme;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let show_welcome = use_state(|| true);
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

    let toggle_welcome = {
        let show_welcome = show_welcome.clone();
        Callback::from(move |_| {
            show_welcome.set(false);
        })
    };

    let toggle_theme = {
        let user = user.clone();
        Callback::from(move |_| {
            let current_theme = *user.theme.borrow();
            let new_theme = match current_theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
            *user.theme.borrow_mut() = new_theme;
        })
    };

    let current_theme = *user.theme.borrow();
    let bg_class = match current_theme {
        Theme::Light => "bg-gray-800",
        Theme::Dark => "bg-gray-900",
    };

    let text_class = match current_theme {
        Theme::Light => "text-gray-200",
        Theme::Dark => "text-white",
    };
    
    if *show_welcome {
        html! {
            <div class={format!("{} flex w-screen", bg_class)}>
                <div class="container mx-auto flex flex-col justify-center items-center h-screen">
                    <div class={format!("text-4xl font-bold mb-8 {}", text_class)}>{"Welcome to YewChat! 💬"}</div>
                    <p class={format!("text-xl mb-12 {}", text_class)}>{"A simple chat application built with Rust and Yew"}</p>
                    <button 
                        onclick={toggle_welcome}
                        class="px-8 py-4 rounded-lg bg-violet-600 text-white font-bold uppercase hover:bg-violet-700 transition-colors"
                    >
                        {"Get Started"}
                    </button>
                    <button 
                        onclick={toggle_theme}
                        class="mt-6 px-4 py-2 rounded-lg bg-gray-600 text-white text-sm hover:bg-gray-700 transition-colors"
                    >
                        {if current_theme == Theme::Light { "Switch to Dark Mode" } else { "Switch to Light Mode" }}
                    </button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class={format!("{} flex w-screen", bg_class)}>
                <div class="container mx-auto flex flex-col justify-center items-center">
                    <div class={format!("text-2xl mb-6 {}", text_class)}>{"Enter your username to start chatting"}</div>
                    <form class="m-4 flex">
                        <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username"/>
                        <Link<Route> to={Route::Chat}> 
                            <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r hover:bg-violet-700 transition-colors" >
                                {"Go Chatting!"}
                            </button>
                        </Link<Route>>
                    </form>
                    <button 
                        onclick={toggle_theme}
                        class="mt-6 px-4 py-2 rounded-lg bg-gray-600 text-white text-sm hover:bg-gray-700 transition-colors"
                    >
                        {if current_theme == Theme::Light { "Switch to Dark Mode" } else { "Switch to Light Mode" }}
                    </button>
                </div>
            </div>
        }
    }
}