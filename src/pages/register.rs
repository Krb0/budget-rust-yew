use yew::{html, function_component, Html};

use crate::{layout::Layout, components::login::form::form_layout::FormLayout};
#[function_component(Register)]
pub fn register()-> Html{
    html!{
        <Layout>
            <FormLayout is_login={false}/>
        </Layout>
    }
}