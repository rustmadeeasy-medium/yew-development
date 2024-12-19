use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class = "container">
            <div class = "row min-vh-100 justify-content-center align">
                <div class = "col-md-4">
                    <form>
                        <label> {"Username"} </label>
                        <input type = "text" />
                        <label> {"Password"} </label>
                        <input type = "text" />
                        <button type = "submit"> {"Login"} </button>
                    </form>
                </div>
            </div>
        </div>
    }
} 