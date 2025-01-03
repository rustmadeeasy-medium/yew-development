use yew::prelude::*;
use gloo_console::log;

use crate::components::input::*;
use crate::components::test::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    // State to manage whether the <Test /> component is visible
    let show_test = use_state(|| false);

    // Handle showing the <Test /> component
    let handle_click = {
        let show_test = show_test.clone();
        Callback::from(move |_| {
            log!("I clicked!");
            show_test.set(true);
        })
    };

    // Handle hiding the <Test /> component (triggered by the child component)
    let hide_test = {
        let show_test = show_test.clone();
        Callback::from(move |_| {
            show_test.set(false);
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
                <Test on_close={hide_test} />
            }
        </form>
    }
}
