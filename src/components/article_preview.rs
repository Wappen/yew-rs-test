use yew::prelude::*;

use crate::model::ArticleModel;
use crate::Card;
use crate::MainRoute;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub article: ArticleModel,
}

#[function_component(ArticlePreview)]
pub fn article_preview(props: &Props) -> Html {
    html! {
        <Card<MainRoute>
                title={ props.article.title.clone() }
                subtitle={ props.article.subtitle() }
                thumbnail={ props.article.thumbnail.clone() }
                to={ MainRoute::Article { id: props.article.id } }>
            { props.article.description.clone() }
        </Card<MainRoute>>
    }
}
