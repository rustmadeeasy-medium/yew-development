use yew::prelude::*; 

#[function_component(App)]
fn app() -> Html {
    html! {
        <div> 
            <p> {"Test"} </p>
        </div>
    }
} 

fn main() {
    yew::Renderer::<App>::new().render();
}
