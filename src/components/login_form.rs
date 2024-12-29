use yew::prelude::*;

use crate::components::input::*;

#[function_component(LoginForm)]  
pub fn login_form() -> Html {
    let onsubmit = Callback::from(|e : SubmitEvent | {
        e.prevent_default(); 
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
                <button type = "submit"> {"Login"} </button>
            </div>
        </form>
    }
}    
                   