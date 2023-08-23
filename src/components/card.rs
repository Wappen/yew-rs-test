use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<R: Routable + 'static> {
    pub title: String,
    pub subtitle: Option<String>,
    pub thumbnail: Option<String>,
    pub to: Option<R>,
}

#[function_component(Card)]
pub fn card<R: Routable + 'static>(props: &Props<R>) -> Html {
    let thumbnail = props.thumbnail.as_ref().map_or(html! {}, |thumbnail| {
        html! {
            <img src={ thumbnail.clone() } alt="..." />
        }
    });

    let body = html! {
        <div class="card-body">
            <h2 class="card-title">{ &props.title }</h2>
            {
                props.subtitle.as_ref().map_or(html! {}, |subtitle| html! {
                    <h6 class="card-subtitle">{ subtitle }</h6>
                })
            }
        </div>
    };

    html! {
        <Link<R> classes="card article-preview" to={ props.to.clone().unwrap_or(Routable::not_found_route().unwrap()) }>
            <div class="card-img-top thumbnail">
                { thumbnail }
            </div>
            { body }
        </Link<R>>
    }
}
