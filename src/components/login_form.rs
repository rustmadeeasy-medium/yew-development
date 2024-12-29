use yew::prelude::*;
use gloo_console::log;

use crate::components::input::*;
use crate::components::test::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    // State to manage whether the <Test /> component should be displayed
    let show_test = use_state(|| false);

    // Handle button click
    let handle_click = {
        let show_test = show_test.clone();
        Callback::from(move |_| {
            log!("I clicked!");
            show_test.set(true); // Update the state to show the <Test /> component
        })
    };

    // Handle form submission
    let onsubmit = Callback::from(|e: SubmitEvent| {
        e.prevent_default(); // Prevent the default form submission
    });

    html! {
        <form {onsubmit}>
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="username"
                    label="Username"
                />
                <div class="mb-3">
                    <Input
                        input_type="password"
                        name="password"
                        label="Password"
                    />
                </div>
                <button onclick={handle_click} type="button">{"Login"}</button>
            </div>

            // Conditionally render the <Test /> component if `show_test` is true
            if *show_test {
                <Test />
            }
        </form>
    }
}
