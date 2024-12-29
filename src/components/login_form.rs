use yew::prelude::*;
use gloo_console::log;


use crate::components::input::*;

fn handleclick() {
    log!("I clicked !")
}

#[function_component(LoginForm)]  
pub fn login_form() -> Html {
    let onsubmit = Callback::from(|e : SubmitEvent | {
        e.prevent_default(); 
    });

    let onclick = Callback::from(|MouseEvent| {
        handleclick();
    });

    html! {
        <form onsubmit = {onsubmit}>
            <div class = "mb-3">
                <Input 
                    input_type = "text" 
                    name = "username" 
                    label = "Username"
                />
                <div class = "mb-3">
                    <Input 
                        input_type = "password" 
                        name = "password" 
                        label = "Password"
                    />
                </div>
                <button onclick = {onclick} type = "submit"> {"Login"} </button>
            </div>
        </form>
    }
}    
                   