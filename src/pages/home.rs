use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

use crate::layout::Layout;
use crate::utils::navigator_redirect;
use crate::pages::Route;
#[function_component(Home)]
pub fn home() -> Html {
     let navigator = use_navigator().unwrap();
     
     let redirect_register = navigator_redirect(navigator, Route::Register);
    html! {
        <div>
            <Layout>
                <div class="lg:2/6 xl:w-2/4 mt-20 lg:mt-40 lg:ml-16 text-left">
                    <div class="text-6xl font-semibold text-gray-900 leading-none">
                        {"Take your finances to next level"}
                    </div>
                    <div class="mt-6 text-xl font-light text-true-gray-500 antialiased">
                        {"Manage your money like a pro!"}
                    </div>
                    <button onclick={redirect_register} class="mt-6 px-8 py-4 rounded-full font-normal tracking-wide bg-gradient-to-b from-blue-600 to-blue-700 text-white outline-none focus:outline-none hover:shadow-lg transition duration-200 ease-in-out">
                        {"Sign Up"}
                    </button>
                </div>
            </Layout>
        </div>
    }
}
