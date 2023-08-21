use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub brand: Option<&'static str>,
    pub logo: Option<&'static str>,
    pub children: ChildrenWithProps<NavbarItem>,
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let logo_html = props.logo.map_or(html! {}, |logo| {
        html! {
            <img src={logo} class={classes!("d-inline-block", "navbar-logo", props.brand.map(|_| "logo-margin"))} alt="Logo" />
        }
    });

    let brand_html = props.brand.map_or(html! {}, |brand| {
        html! {
            <span class="navbar-brand">{ brand }</span>
        }
    });

    let base_html = if props.logo.is_some() || props.brand.is_some() {
        html! {
            <Link<MainRoute> classes="navbar-brand" to={ MainRoute::Home }>
                { logo_html }
                { brand_html }
            </Link<MainRoute>>
        }
    } else {
        html! {}
    };

    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container-fluid">
                { base_html }
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav">
                        { props.children.clone() }
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct NavbarItemProps {
    pub name: &'static str,
    pub to: MainRoute,
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &NavbarItemProps) -> Html {
    let current_link = use_route::<MainRoute>().unwrap();
    let active = current_link == props.to;
    let classes = classes!("nav-link", if active { "active" } else { "" });

    html! {
        <li class="nav-item">
            <Link<MainRoute> classes={ classes } to={ props.to.clone() }><span>{ props.name }</span></Link<MainRoute>>
        </li>
    }
}
