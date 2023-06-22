use yew::{function_component, Properties, Html, html, UseStateHandle};


#[function_component(Loading)]
pub fn loading_component(props:&Props) -> Html{
    let copyy = props.is_loading.clone().to_string();
    html!{
        if copyy == "true" {
            <div>
                {"Loading..."}
            </div>
        }else{
            <div>
                {props.text.to_owned()}
            </div>
        }
    }
}


#[derive(Properties, PartialEq)]
pub struct Props{
    pub text: String,
    pub is_loading: UseStateHandle<bool>
}