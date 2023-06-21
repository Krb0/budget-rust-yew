use yew::{Callback, UseStateHandle, MouseEvent};

pub fn _set_state<T:'static + Clone>(state: UseStateHandle<T>, new_state: T) -> yew::Callback<MouseEvent>{

    Callback::from(move|_:MouseEvent| {
        state.set(new_state.clone());
    })
}