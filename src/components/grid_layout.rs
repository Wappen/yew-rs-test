use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub columns: i32,
    pub children: Children,
}

#[function_component(GridLayout)]
pub fn grid_layout(props: &Props) -> Html {
    html! {
        <div class="container">
            {
                props.children.iter().collect::<Vec<Html>>().chunks(props.columns as _).map(|c| {
                    html! {
                        <div class="row">
                            {
                                c.iter().map(|item| {
                                    html! {
                                        <div class="col">
                                            { item.clone() }
                                        </div>
                                    }
                                }).collect::<Vec<Html>>()
                            }
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
