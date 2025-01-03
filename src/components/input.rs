use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label : AttrValue,
    pub input_type : AttrValue,
    pub name : AttrValue,
}

#[function_component(Input)]
pub fn input(props : &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <div>
            <label id = {html_id.clone()}> {props.label.clone()} </label>
            <input 
                type = {props.input_type.clone()} 
                name = {props.name.clone()}    
            />
        </div>
    }
}