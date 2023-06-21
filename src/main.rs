mod layout; 
mod utils;
mod components;
mod pages;
use pages::Router as Router;
use yew::prelude::*;



#[function_component(App)]
fn app() -> Html{
    html!{
            <Router/>
    }
}




fn main(){
    yew::Renderer::<App>::new().render();
}



/* #[derive(Clone)]
struct Model {
    value: i64
} */
/* let state = use_state(|| Model { value: 0 });
    let onclick = {
        let state = state.clone();
        let new_state = Model{
            value: state.value + 1
        };
        
        set_state(state, new_state)
    }; */