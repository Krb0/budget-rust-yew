use yew::{Callback, UseStateHandle, MouseEvent};
use yew_router::prelude::Navigator;

use crate::pages::Route;

pub fn _set_state<T:'static + Clone>(state: UseStateHandle<T>, new_state: T) -> yew::Callback<MouseEvent>{

    Callback::from(move|_:MouseEvent| {
        state.set(new_state.clone());
    })
}

pub fn navigator_redirect(original_navigator: Navigator, route:Route) -> yew::Callback<MouseEvent>{
    let navigator: Navigator = original_navigator.to_owned();
    Callback::from(move|_: MouseEvent| navigator.push(&route))
}