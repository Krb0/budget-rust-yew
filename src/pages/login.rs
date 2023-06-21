use yew::{html, function_component, Html};

use crate::{layout::Layout};
#[function_component(Login)]
pub fn login()-> Html{
    html!{
        <Layout>
            <h1>{"Login"}</h1>
        </Layout>
    }
}