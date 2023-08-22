use crate::AppContext;
use yew::prelude::*;

use crate::model::ArticleId;

use crate::Markdown;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: ArticleId,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let ctx = use_context::<AppContext>().unwrap();
    let article = ctx.article_repo.find_by_id(props.id).unwrap();

    let subtitle = match article.published {
        Some(published) => format!("by {} on {}", article.author, published.format("%Y/%m/%d")),
        None => format!("by {}", article.author),
    };

    html! {
        <div class="container">
            <h1>{ article.title }</h1>
            <h2 class="article-subtitle">{ subtitle }</h2>
            <Markdown markdown={ article.text } />
        </div>
    }
}
