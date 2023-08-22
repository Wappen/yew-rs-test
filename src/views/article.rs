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

    html! {
        <div class="container article">
            <h1 class="article-title my-4">{ &article.title }</h1>
            <h2 class="article-subtitle mb-3">{ article.subtitle() }</h2>
            <img class="article-header-img" src={ article.header_img.clone() } />
            <Markdown classes={ classes!("container-sm", "article-content") } markdown={ article.text } />
        </div>
    }
}
