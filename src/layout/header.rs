
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

use crate::{pages::Route, components::shared::logo::Logo, utils::navigator_redirect};


#[function_component(LayoutHeader)]
pub fn render() -> Html{

    let navigator = use_navigator().unwrap();
     
    let redirect_signup = navigator_redirect(&navigator, Route::Register);
    let redirect_login = navigator_redirect(&navigator, Route::Login);
    let redirect_home = navigator_redirect(&navigator, Route::Home);


    html!{
        <div class="bg-white">
            <div class="border py-3 px-6">
                <div class="flex justify-between">
                    <div class="flex items-center cursor-pointer" onclick={redirect_home}>
                    <Logo />
                        <span class="ml-2 font-semibold text-[#252C32]">
                            {"BudgetApp"}
                        </span>
                    </div>
                    <div class="ml-2 flex">
                        <button onclick={redirect_login} class="ml-2 flex cursor-pointer items-center gap-x-1 rounded-md border py-2 px-4 hover:bg-gray-100 transition duration-150 hover:text-black bg-[#1d6ed1] text-white">
                            <span class="text-sm font-medium">{"Log in"}</span>
                        </button>
                        <button onclick={redirect_signup} class="ml-2 flex cursor-pointer items-center gap-x-1 rounded-md border py-2 px-4 hover:bg-gray-100 ">
                            <span class="text-sm font-medium">{"Sign up"}</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
