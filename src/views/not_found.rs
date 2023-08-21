use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="container">
            <h1>{ "404 Not Found" }</h1>
        </div>
    }
}
