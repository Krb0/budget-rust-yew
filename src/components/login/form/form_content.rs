use yew::{function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::{utils::navigator_redirect, pages::Route};

#[function_component(FormContent)]
pub fn login_form(props: &Props) -> Html {
    let button_text: String = if props.is_login {
        "Login".to_owned()
    } else {
        "Register".to_owned()
    };
    let already_registered_text: String = if props.is_login {
        "You don't have an account?".to_owned()
    } else {
        "Already have an account?".to_owned()
    };

    let navigator = use_navigator().unwrap();
    let route_redirect = if props.is_login {
        Route::Register
    } else {
        Route::Login
    };
    let redirect: yew::Callback<yew::MouseEvent> = navigator_redirect(navigator, route_redirect); 
    html! {
        <>
            <div class="mt-10">
                <form action="#">
                <div class="flex flex-col mb-6">
                    <label for="email" class="mb-1 text-xs sm:text-sm tracking-wide text-gray-600">{"E-Mail Address:"}</label>
                    <div class="relative">
                    <div class="inline-flex items-center justify-center absolute left-0 top-0 h-full w-10 text-gray-400">
                        <svg class="h-6 w-6" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                        </svg>
                    </div>

                    <input id="email" type="email" name="email" class="text-sm sm:text-base placeholder-gray-500 pl-10 pr-4 rounded-lg border border-gray-400 w-full py-2 focus:outline-none focus:border-blue-400" placeholder="E-Mail Address" />
                    </div>
                </div>
                <div class="flex flex-col mb-6">
                    <label for="password" class="mb-1 text-xs sm:text-sm tracking-wide text-gray-600">{"Password:"}</label>
                    <div class="relative">
                    <div class="inline-flex items-center justify-center absolute left-0 top-0 h-full w-10 text-gray-400">
                        <span>
                        <svg class="h-6 w-6" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                            <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                        </svg>
                        </span>
                    </div>

                    <input id="password" type="password" name="password" class="text-sm sm:text-base placeholder-gray-500 pl-10 pr-4 rounded-lg border border-gray-400 w-full py-2 focus:outline-none focus:border-blue-400" placeholder="Password" />
                    </div>
                </div>

                <div class="flex items-center mb-6 -mt-4">
                    <div class="flex ml-auto">
                    <a href="#" class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">{"Forgot Your Password?"}</a>
                    </div>
                </div>

                <div class="flex w-full">
                    <button type="submit" class="flex items-center justify-center focus:outline-none text-white text-sm sm:text-base bg-blue-600 hover:bg-blue-700 rounded py-2 w-full transition duration-150 ease-in">
                    <span class="mr-2 uppercase">{button_text}</span>
                    <span>
                        <svg class="h-6 w-6" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M13 9l3 3m0 0l-3 3m3-3H8m13 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </span>
                    </button>
                </div>
                </form>
            </div>
            <div class="flex justify-center items-center mt-6">
                <a href="#" target="_blank" class="inline-flex items-center font-bold text-blue-500 hover:text-blue-700 text-xs text-center">
                <span>
                    <svg class="h-6 w-6" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                    <path d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                    </svg>
                </span>
                <span class="ml-2" onclick={redirect}>{already_registered_text}</span>
                </a>
            </div>
          </>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_login: bool,
}