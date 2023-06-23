
use yew::{function_component, html, Html, Properties, Callback, SubmitEvent, MouseEvent, use_node_ref, use_state};
use yew_router::prelude::{use_navigator, Navigator};
use web_sys::HtmlInputElement;
use crate::{utils::navigator_redirect,utils::request_handler::request_post, pages::Route};
use serde::{Deserialize, Serialize};
use gloo_console::log;
use serde_json;

use crate::{components::shared::loading::Loading};

#[function_component(FormContent)]
pub fn login_form(props: &Props) -> Html {
    let navigator = &use_navigator().unwrap();
    let (already_registered_text, button_text, redirect) = get_static_data(props, navigator);
    let email_input_ref = use_node_ref();
    let password_input_ref = use_node_ref();

    let loading_state = use_state(|| false);
    let loading_state_clone = loading_state.clone();

    let loading_class = if *loading_state_clone {
        "loading-true"
    } else {
        ""
    };


    let on_submit:Callback<SubmitEvent> = {
        let navigator = navigator.clone();
        let email_ref = email_input_ref.clone();
        let password_ref = password_input_ref.clone();
        let loading_state_clone_for_closure = loading_state_clone.clone();
        let is_login = props.is_login.clone();
        Callback::from(move |ev: SubmitEvent| {
            let navigator_submit = navigator.clone();
            let push_closure = move || {
                let _ = navigator_submit.push(&Route::Home);
            };
            ev.prevent_default();
            if loading_state_clone_for_closure.to_string() == "true" {
                log!("dddd...");
                return;
            }
            let loading = loading_state.clone();
            loading.set(true);

            let email = email_ref.cast::<HtmlInputElement>().expect("Email must exist").value();
            let password = password_ref.cast::<HtmlInputElement>().expect("Password must exist").value();
            let request_body = LoginRequestBody { username:email, password };
            let body = serde_json::to_string(&request_body).unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                let endpoint = if is_login {"https://dummyjson.com/auth/login"} else {"https://dummyjson.com/auth/signup"};
                    let res = request_post::<LoginResponse>(&endpoint.to_string(), &body).await;
                    if res.is_err() {
                        log!("Error: {:?}");
                        loading.set(false);
                        return;
                    }
                    loading.set(false);
                    let data = res.unwrap();
                   
                    
                        push_closure();
                   
                   log!("Name: {:?}", data.firstName);
            });

        })
    };
    html! {
        <>
            <div class="mt-10">
                <form onsubmit={on_submit} autocomplete="on">
                <div class="flex flex-col mb-6">
                    <label for="email" class="mb-1 text-xs sm:text-sm tracking-wide text-gray-600">{"E-Mail Address:"}</label>
                    <div class="relative">
                    <div class="inline-flex items-center justify-center absolute left-0 top-0 h-full w-10 text-gray-400">
                        <svg class="h-6 w-6" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                        </svg>
                    </div>

                    <input value="kminchelle" autocomplete="email" ref={email_input_ref} id="email" type="text" name="email" class="text-sm sm:text-base placeholder-gray-500 pl-10 pr-4 rounded-lg border border-gray-400 w-full py-2 focus:outline-none focus:border-blue-400" placeholder="E-Mail Address" />
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

                    <input value="0lelplR" autocomplete="current-password" ref={password_input_ref} id="password" type="password" name="password" class="text-sm sm:text-base placeholder-gray-500 pl-10 pr-4 rounded-lg border border-gray-400 w-full py-2 focus:outline-none focus:border-blue-400" placeholder="Password" />
                    </div>
                </div>

                <div class="flex items-center mb-6 -mt-4">
                    <div class="flex ml-auto">
                    <a href="#" class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">{"Forgot Your Password?"}</a>
                    </div>
                </div>

                <div class="flex w-full">
                    <button type="submit" class={format!("flex items-center justify-center focus:outline-none text-white text-sm sm:text-base bg-blue-600 hover:bg-blue-700 rounded py-2 w-full transition duration-150 ease-in {}", loading_class)} >
                    <span class={"mr-2 uppercase"}>  
                            <Loading text={button_text} is_loading={loading_state_clone} />
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


fn get_static_data(props:&Props, navigator: &Navigator) -> (String, String, Callback<MouseEvent>){
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

    let route_redirect = if props.is_login {
        Route::Register
    } else {
        Route::Login
    };
    let redirect: Callback<MouseEvent> = navigator_redirect(&navigator, route_redirect); 

    
    (already_registered_text, button_text, redirect)
}

#[derive(Serialize, Deserialize)]
struct LoginRequestBody {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginResponse {
    id: u32,
    username: String,
    email: String,
    firstName: String,
    lastName: String,
    gender: String,
    image: String,
    token: String,
}
