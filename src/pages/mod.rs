mod home;
mod login;
mod register;

use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::{
    pages::home::Home,
    pages::login::Login,
    pages::register::Register,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Register
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { <Login/> },
        Route::Register => html! { <Register/> },
    }
}


#[function_component(Router)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}