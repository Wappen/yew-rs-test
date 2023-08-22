use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod model;
mod repositories;
mod views;

use components::*;
use model::*;
use repositories::*;
use views::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/article/:id")]
    Article { id: ArticleId },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! { <Home /> },
        MainRoute::About => html! { <About /> },
        MainRoute::Article { id } => html! { <Article id={id} /> },
        MainRoute::NotFound => html! { <NotFound /> },
    }
}

struct AppContext {
    pub article_repo: Rc<dyn ArticleRepository>,
}

impl AppContext {
    pub fn mocking() -> Self {
        Self {
            article_repo: Rc::new(MockArticleRepository::default()),
        }
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            article_repo: self.article_repo.clone(),
        }
    }
}

impl PartialEq for AppContext {
    fn eq(&self, other: &Self) -> bool {
        self.article_repo.eq(&other.article_repo)
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <ContextProvider<AppContext> context={AppContext::mocking()}>
                <BrowserRouter>
                    <Navbar brand="Wappen" logo="/static/img/logo.png">
                        <NavbarItem name="Home" to={MainRoute::Home}/>
                        <NavbarItem name="About" to={MainRoute::About}/>
                    </Navbar>
                    <Switch<MainRoute> render={switch_main}/>
                </BrowserRouter>
            </ContextProvider<AppContext>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
