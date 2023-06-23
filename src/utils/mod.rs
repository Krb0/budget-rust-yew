pub mod request_handler;
use yew::{Callback, UseStateHandle, MouseEvent};
use yew_router::prelude::Navigator;

use crate::pages::Route;

pub fn _set_state<T:'static + Clone>(state: UseStateHandle<T>, new_state: T) -> yew::Callback<MouseEvent>{

    Callback::from(move|_:MouseEvent| {
        state.set(new_state.clone());
    })
}

pub fn navigator_redirect(navigator: &Navigator, route:Route) -> yew::Callback<MouseEvent>{
    let navigator_ref = navigator.clone(); 
    Callback::from(move|_: MouseEvent| navigator_ref.push(&route))
}
