use yew::{html, function_component, Html};

use crate::{ layout::Layout};


#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <div>
            <Layout>
                <div>{"Hola"}</div>
            </Layout>
        </div>
    }
}