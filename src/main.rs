use yew::prelude::*; 

mod pages;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        < pages::login::Login />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
