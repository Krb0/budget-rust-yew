use yew::{function_component, Html, html, Properties};

#[function_component(Logo)]
pub fn logo(props: &ContentProps) -> Html{
    let mut fill:String = "#1d6ed1".to_owned();
    let mut width:String = "40px".to_owned();

    if props.fill.is_some(){
        fill = props.fill.clone().unwrap();
    }
    if props.width.is_some(){
        width = props.width.clone().unwrap();
    }
    html!{
        <svg fill={fill} width={width} version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" >
            <path d="M480,240v-48c0-17.674-14.328-32-32-32H96c-17.648,0-32-14.352-32-32s14.352-32,32-32h352c17.672,0,32-14.328,32-32
                s-14.328-32-32-32H96C43.062,32,0,75.063,0,128v32v256c0,35.346,28.656,64,64,64h384c17.672,0,32-14.326,32-32v-48
                c17.672,0,32-14.326,32-32v-96C512,254.326,497.672,240,480,240z M432,352c-17.672,0-32-14.326-32-32s14.328-32,32-32
                c17.672,0,32,14.326,32,32S449.672,352,432,352z"
            />
        </svg>
    }
}


#[derive(Properties, PartialEq,Clone)]
pub struct ContentProps {
    pub width: Option<String>,
    pub fill: Option<String>,
}