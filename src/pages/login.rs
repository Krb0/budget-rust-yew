use yew::{html, function_component, Html};

use crate::{layout::Layout, components::login::form::form_layout::FormLayout};
#[function_component(Login)]
pub fn login()-> Html{
    html!{
        <Layout>
            <FormLayout is_login={true}/>
        </Layout>
    }
}