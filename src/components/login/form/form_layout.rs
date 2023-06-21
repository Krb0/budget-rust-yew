use yew::{html, Html, function_component, Properties};
use crate::components::login::form::form_content::FormContent;

#[function_component(FormLayout)]
pub fn form_layout(props: &Props) -> Html{
    let title_text :String = if props.is_login {"Login".to_owned()} else {"Register".to_owned()};
    let or_text :String = if props.is_login {"Or Log in With Email".to_owned()} else {"Or Sign up With Email".to_owned()};

    html!{
        <div class="min-h-[95vh] flex flex-col items-center justify-center bg-gray-300">
        <div class="flex flex-col bg-white shadow-md px-4 sm:px-6 md:px-8 lg:px-10 py-8 rounded-md w-full max-w-md">
          <div class="font-medium self-center text-xl sm:text-2xl uppercase text-gray-800">{title_text}</div>
          <button class="relative mt-6 border rounded-md py-2 text-sm text-gray-800 bg-gray-100 hover:bg-gray-200">
            <span class="absolute left-0 top-0 flex items-center justify-center h-full w-10 text-blue-500"><i class="fab fa-facebook-f"></i></span>
            <span>{"Enter with Google"}</span>
          </button>
          <div class="relative mt-10 h-px bg-gray-300">
            <div class="absolute left-0 top-0 flex justify-center w-full -mt-2">
              <span class="bg-white px-4 text-xs text-gray-500 uppercase">{or_text}</span>
            </div>
          </div>
            <FormContent is_login={props.is_login}/>
        </div>
      </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub is_login: bool
}