use yew::prelude::*;

use crate::{AppContext, ArticlePreview, GridLayout};

#[function_component(Home)]
pub fn home() -> Html {
    let ctx = use_context::<AppContext>().unwrap();
    let articles = ctx.article_repo.find_all();

    html! {
        <div class="container">
            <h1>{ "Hello, Read My Articles" }</h1>
            <GridLayout columns=4>
                {
                    for articles.iter().map(|article| {
                        html! {
                            <ArticlePreview article={article.clone()} />
                        }
                    }).collect::<Vec<Html>>()
                }
            </GridLayout>
        </div>
    }
}
