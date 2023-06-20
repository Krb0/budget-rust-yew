use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone)]
struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html{
    let state = use_state(|| Model { value: 0 });
    let onclick = {
        let state = state.clone();
        let new_state = Model{
            value: state.value + 1
        };
        set_state(state, new_state)
    };

    html!{
        <div>
        <button {onclick}>{"+1"}</button>
            <h1>{state.value}</h1>
        </div>
    }
}


fn set_state<T:'static + Clone>(state: UseStateHandle<T>, new_state: T) -> yew::Callback<MouseEvent>{

    Callback::from(move|_:MouseEvent| {
        state.set(new_state.clone());
    })
}

fn main(){
    yew::Renderer::<App>::new().render();
}