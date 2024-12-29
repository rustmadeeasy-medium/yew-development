use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Test)]
pub fn test(props: &TestProps) -> Html {
    let on_close = props.on_close.clone();

    html! {
        <div style="display: flex; flex-direction: row; border: 1px solid black; width: 200px;">
            <p>{ "Rewrite it in Rust!" }</p>
            <div
                style="margin-left: 20px; border: 1px solid black; width: 20px; height: 20px; cursor: pointer;"
                onclick={on_close}
            >
                <p>{ "X" }</p>
            </div>
        </div>
    }
}
