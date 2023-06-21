mod header;
use header::LayoutHeader;
use yew::prelude::*;

#[function_component(Layout)]
pub fn render(props: &Props) -> Html {
    html! {
        <div>
            <LayoutHeader />
            <Content items={props.children.clone()} />
        </div>
    }
}

#[function_component(Content)]
fn content(props: &ContentProps) -> Html {
    html! {
        <div>
            {
                props.items.as_ref().map(|children| {
                    if children.is_empty() {
                        html! {
                            <div></div>
                    }
                } else {
                    html! { for children.iter() }
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub items: Option<Children>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Option<Children>,
}
