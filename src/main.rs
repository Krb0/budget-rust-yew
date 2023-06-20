mod layout; // Import the layout module
use layout::Layout; // Import the render function from the layout module

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
            <Layout />
        
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