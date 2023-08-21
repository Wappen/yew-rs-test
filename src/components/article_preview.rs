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
    let subtitle = match props.article.published {
        Some(published) => format!(
            "by {} on {}",
            props.article.author,
            published.format("%Y/%m/%d")
        ),
        None => format!("by {}", props.article.author),
    };

    let description = if props.article.description.len() > 100 {
        format!(
            "{}…",
            &props.article.description[0..props.article.description.len().min(100)].trim()
        )
    } else {
        props.article.description.clone()
    };

    html! {
        <Card<MainRoute>
                title={ props.article.title.clone() }
                subtitle={ subtitle }
                thumbnail={ props.article.thumbnail.clone() }
                to={ MainRoute::Article { id: props.article.id } }>
            { description }
        </Card<MainRoute>>
    }
}
