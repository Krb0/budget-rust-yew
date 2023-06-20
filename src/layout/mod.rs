mod header;
use header::LayoutHeader;
use yew::prelude::*;

#[function_component(Layout)]
pub fn render() -> Html{
    html!{
        <div>
            <LayoutHeader/>
        </div>
    }
}